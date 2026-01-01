use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::{Sender, unbounded};
use std::{
    fs::OpenOptions,
    io::Write,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

const TARGET_SAMPLE_RATE: usize = 16_000;
const CHUNK_SECONDS: usize = 15;
const CHUNK_SAMPLES: usize = TARGET_SAMPLE_RATE * CHUNK_SECONDS;

fn main() -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    let host = cpal::default_host();
    let device = find_device(&host)?;
    let config = device.default_input_config()?;
    let sample_rate = config.sample_rate().0 as usize;
    let channels = config.channels() as usize;

    println!("Using device: {}", device.name()?);
    println!("Input sample rate: {}", sample_rate);

    let (tx, rx) = unbounded::<Vec<f32>>();

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _| audio_callback(data, channels, &tx),
            err_fn,
            None,
        )?,
        _ => panic!("Only f32 sample format supported"),
    };

    stream.play()?;

    // Whisper init
    let model_path = PathBuf::from("model").join("ggml-base.en.bin");
    let ctx = WhisperContext::new_with_params(
        model_path.to_str().expect("Invalid model path"),
        whisper_rs::WhisperContextParameters::default()
    )?;
    let mut state = ctx.create_state()?;
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_language(Some("en"));

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("Notes/transcript.txt")?;

    let mut buffer: Vec<f32> = Vec::new();
    let resample_ratio = if sample_rate != TARGET_SAMPLE_RATE {
        Some(TARGET_SAMPLE_RATE as f64 / sample_rate as f64)
    } else {
        None
    };

    while running.load(Ordering::SeqCst) {
        if let Ok(data) = rx.recv_timeout(Duration::from_millis(100)) {
            let processed_data = if let Some(ratio) = resample_ratio {
                linear_resample(&data, ratio)
            } else {
                data
            };

            buffer.extend_from_slice(&processed_data);

            if buffer.len() >= CHUNK_SAMPLES {
                println!("Transcribing {} samples ({} seconds)...", buffer.len(), buffer.len() as f64 / TARGET_SAMPLE_RATE as f64);
                
                // Normalize audio to prevent clipping
                let max_val = buffer.iter().fold(0.0f32, |acc, &x| acc.max(x.abs()));
                if max_val > 0.0 {
                    let scale = (0.95 / max_val).min(1.0);
                    for sample in buffer.iter_mut() {
                        *sample *= scale;
                    }
                }
                
                let mut transcribe_params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
                transcribe_params.set_language(Some("en"));
                transcribe_params.set_translate(false);
                transcribe_params.set_print_progress(false);
                transcribe_params.set_print_special(false);
                transcribe_params.set_print_realtime(false);
                transcribe_params.set_suppress_blank(true);
                transcribe_params.set_suppress_non_speech_tokens(true);
                
                state.full(transcribe_params, &buffer)?;
                let n = state.full_n_segments()?;

                println!("Found {} segments", n);
                for i in 0..n {
                    let text = state.full_get_segment_text(i)?;
                    if !text.trim().is_empty() {
                        writeln!(file, "{}", text)?;
                        file.flush()?;
                        println!("  [{}] {}", i, text);
                    }
                }

                buffer.clear();
            }
        }
    }

    println!("Shutting down cleanly.");
    Ok(())
}

fn audio_callback(input: &[f32], channels: usize, tx: &Sender<Vec<f32>>) {
    let mono = stereo_to_mono(input, channels);
    let _ = tx.send(mono);
}

fn stereo_to_mono(input: &[f32], channels: usize) -> Vec<f32> {
    input
        .chunks(channels)
        .map(|c| c.iter().sum::<f32>() / channels as f32)
        .collect()
}

fn find_device(host: &cpal::Host) -> Result<cpal::Device> {
    // Try to find a preferred device first
    for device in host.devices()? {
        let name = device.name()?;
        if cfg!(target_os = "macos") && name.contains("BlackHole") {
            println!("Found preferred device: {}", name);
            return Ok(device);
        }
        if cfg!(target_os = "windows") {
            let name_lower = name.to_lowercase();
            // Look for common virtual audio cable names or loopback devices
            if name_lower.contains("cable") || name_lower.contains("loopback") || 
               name_lower.contains("virtual") || name_lower.contains("stereo mix") {
                println!("Found preferred device: {}", name);
                return Ok(device);
            }
        }
    }
    
    // Fall back to default input device
    let default = host
        .default_input_device()
        .expect("No suitable audio device found");
    println!("Using default input device: {}", default.name()?);
    Ok(default)
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("Stream error: {}", err);
}

fn linear_resample(input: &[f32], ratio: f64) -> Vec<f32> {
    if input.is_empty() {
        return Vec::new();
    }
    
    let output_len = (input.len() as f64 * ratio).round() as usize;
    let mut output = Vec::with_capacity(output_len);
    
    for i in 0..output_len {
        let src_pos = i as f64 / ratio;
        let src_index = src_pos.floor() as usize;
        let frac = src_pos - src_pos.floor();
        
        if src_index + 1 < input.len() {
            // Linear interpolation between two samples
            let frac_f32 = frac as f32;
            let sample = input[src_index] * (1.0 - frac_f32) + input[src_index + 1] * frac_f32;
            output.push(sample);
        } else if src_index < input.len() {
            // Last sample, no interpolation possible
            output.push(input[src_index]);
        }
    }
    
    output
}
