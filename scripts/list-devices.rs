// Quick utility to list available audio devices

use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    let host = cpal::default_host();
    
    println!("Available audio input devices:");
    println!("Platform: {:?}", host.id());
    println!("---");
    
    match host.input_devices() {
        Ok(devices) => {
            for (i, device) in devices.enumerate() {
                if let Ok(name) = device.name() {
                    println!("[{}] {}", i, name);
                    if let Ok(config) = device.default_input_config() {
                        println!("    Sample rate: {} Hz", config.sample_rate().0);
                        println!("    Channels: {}", config.channels());
                        println!("    Format: {:?}", config.sample_format());
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error enumerating devices: {}", e);
        }
    }
    
    println!("\nDefault input device:");
    if let Some(device) = host.default_input_device() {
        if let Ok(name) = device.name() {
            println!("  {}", name);
        }
    } else {
        println!("  None found");
    }
}

