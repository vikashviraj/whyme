# WhyMe

A real-time audio transcription tool that captures system audio and transcribes it using OpenAI's Whisper model. Perfect for meeting notes, lecture recordings, or any scenario where you need automatic transcription of audio playing on your computer.

## Features

- 🎤 **Real-time Audio Capture**: Captures audio from your system or microphone
- 📝 **Automatic Transcription**: Uses Whisper AI for accurate speech-to-text conversion
- 🔄 **Automatic Resampling**: Handles different audio sample rates automatically
- 💾 **Persistent Storage**: Saves transcriptions to a text file for easy access
- 🖥️ **Cross-Platform**: Works on macOS, Windows, and Linux
- ⚡ **Efficient**: Processes audio in 15-second chunks for optimal performance
- 🎯 **Smart Device Detection**: Automatically finds and uses virtual audio devices

## Requirements

### System Requirements

- **Rust**: 1.70 or later ([Install Rust](https://www.rust-lang.org/tools/install))
- **Cargo**: Comes with Rust installation
- **Whisper Model**: `ggml-base.en.bin` (included in repository, or download from [OpenAI Whisper](https://github.com/openai/whisper))

### Platform-Specific Requirements

#### macOS
- macOS 10.15+ (Catalina or later)
- For system audio capture: [BlackHole](https://github.com/ExistentialAudio/BlackHole) (recommended)

#### Windows
- Windows 10 or later
- For system audio capture: [VB-Audio Cable](https://vb-audio.com/Cable/) or enable "Stereo Mix" in sound settings

#### Linux
- ALSA or PulseAudio
- For system audio capture: Configure PulseAudio loopback

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/vikashviraj/whyme.git
cd whyme
```

### 2. Download the Whisper Model

The model file should be placed in the `model/` directory:

```bash
# Create model directory if it doesn't exist
mkdir -p model

# Download the base English model (if not already present)
# You can download from: https://huggingface.co/ggerganov/whisper.cpp
# Or directly from: https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin
```

**Note**: The repository includes `model/ggml-base.en.bin` by default. If you need a different model or language, replace this file.

### 3. Build the Project

#### Using the Build Script (Recommended)

```bash
# Make the build script executable (macOS/Linux)
chmod +x build.sh

# Build and run
./build.sh run
```

#### Using Cargo Directly

```bash
# Build
cargo build --release

# Run
cargo run --release
```

#### macOS-Specific Build Notes

If you encounter C++ compilation errors on macOS, the `build.sh` script automatically sets the required environment variables. For manual builds:

```bash
export CXXFLAGS="-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/c++/v1"
export MACOSX_DEPLOYMENT_TARGET=15.0
cargo build --release
```

## Usage

### Basic Usage

1. **Start the application**:
   ```bash
   ./build.sh run
   # or
   cargo run --release
   ```

2. **Select your audio source**:
   - The application will automatically detect and use:
     - **macOS**: BlackHole (if installed) or default microphone
     - **Windows**: VB-Audio Cable, Loopback devices, or default microphone
     - **Linux**: Default audio input device

3. **View transcriptions**:
   - Transcriptions are saved to `Notes/transcript.txt`
   - Console output shows real-time transcription progress
   - Press `Ctrl+C` to stop gracefully

### Listing Available Audio Devices

To see what audio devices are available on your system:

```bash
cargo run --bin list-devices
```

This is useful for troubleshooting or selecting a specific device.

### Configuration

#### Changing the Output Directory

Edit `src/main.rs` and modify the transcript file path:

```rust
.open("Notes/transcript.txt")?;  // Change this path
```

#### Adjusting Chunk Size

The default chunk size is 15 seconds. To change it, modify the constant in `src/main.rs`:

```rust
const CHUNK_SECONDS: usize = 15;  // Change this value
```

#### Changing the Model

Replace `model/ggml-base.en.bin` with a different Whisper model:
- `ggml-tiny.en.bin` - Fastest, least accurate
- `ggml-base.en.bin` - Balanced (default)
- `ggml-small.en.bin` - Better accuracy
- `ggml-medium.en.bin` - High accuracy
- `ggml-large-v2.bin` - Best accuracy (multilingual)

## Platform-Specific Setup

### macOS: Capturing System Audio

1. **Install BlackHole**:
   ```bash
   brew install blackhole-16ch
   # or download from: https://github.com/ExistentialAudio/BlackHole
   ```

2. **Configure Audio Routing**:
   - Open "Audio MIDI Setup" (Applications > Utilities)
   - Create a Multi-Output Device including:
     - Your speakers/headphones
     - BlackHole 16ch
   - Set this as your system output
   - WhyMe will automatically detect and use BlackHole

### Windows: Capturing System Audio

1. **Option A: VB-Audio Cable** (Recommended)
   - Download and install [VB-Audio Cable](https://vb-audio.com/Cable/)
   - Set VB-Audio Cable as your default playback device
   - WhyMe will automatically detect and use it

2. **Option B: Stereo Mix**
   - Right-click the speaker icon → "Sounds"
   - Go to "Recording" tab
   - Right-click and enable "Show Disabled Devices"
   - Enable "Stereo Mix"
   - WhyMe will detect and use it automatically

### Linux: Capturing System Audio

Configure PulseAudio loopback:

```bash
# Load loopback module
pactl load-module module-loopback

# Or create a null sink
pactl load-module module-null-sink sink_name=virtual_speaker
```

## Troubleshooting

### No Audio Devices Found

**Problem**: Application can't find any audio input devices.

**Solutions**:
- Run `cargo run --bin list-devices` to see available devices
- Check that your audio drivers are installed and working
- On Windows, ensure "Stereo Mix" or virtual audio cable is enabled
- On macOS, verify BlackHole is installed and running

### Poor Transcription Quality

**Problem**: Transcriptions are inaccurate or contain repeated words.

**Solutions**:
- Ensure audio levels are adequate (not too quiet or too loud)
- Check that the correct audio source is selected
- Try a larger Whisper model (e.g., `ggml-small.en.bin`)
- Verify the audio isn't being resampled incorrectly

### Build Errors on macOS

**Problem**: C++ compilation errors related to `<atomic>` header.

**Solution**: Use the provided `build.sh` script, which sets the correct environment variables:

```bash
./build.sh build
```

### Model Not Found

**Problem**: Error about missing model file.

**Solution**: Ensure `model/ggml-base.en.bin` exists in the project root:

```bash
ls model/ggml-base.en.bin
```

If missing, download from [Hugging Face](https://huggingface.co/ggerganov/whisper.cpp) or use the included model.

### High CPU Usage

**Problem**: Application uses too much CPU.

**Solutions**:
- Use a smaller Whisper model (`ggml-tiny.en.bin`)
- Increase `CHUNK_SECONDS` to process larger chunks less frequently
- Close other resource-intensive applications

## Architecture

### Components

- **Audio Capture**: Uses `cpal` for cross-platform audio input
- **Resampling**: Custom linear interpolation for sample rate conversion
- **Transcription**: `whisper-rs` bindings to OpenAI's Whisper model
- **Storage**: Simple file-based output to `Notes/transcript.txt`

### Audio Processing Pipeline

1. Audio captured from input device (system audio or microphone)
2. Converted to mono if multi-channel
3. Resampled to 16kHz if needed (Whisper's required sample rate)
4. Buffered in 15-second chunks
5. Normalized to prevent clipping
6. Sent to Whisper for transcription
7. Results written to transcript file

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes** with clear, documented code
4. **Test on your platform** before submitting
5. **Commit with clear messages**: `git commit -m "Add amazing feature"`
6. **Push to your fork**: `git push origin feature/amazing-feature`
7. **Open a Pull Request**

### Development Setup

```bash
# Clone your fork
git clone https://github.com/vikashviraj/whyme.git
cd whyme

# Create a branch
git checkout -b feature/your-feature

# Make changes and test
cargo test
cargo run --release

# Submit PR
```

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy: `cargo clippy`
- Ensure tests pass: `cargo test`

## Testing

### Local Testing

```bash
# Run tests
cargo test

# Check compilation
cargo check

# List audio devices (useful for testing)
cargo run --bin list-devices
```

### Cross-Platform Testing

The repository includes GitHub Actions workflows for automated Windows testing. Push to GitHub to trigger automated builds on Windows.

## Performance

- **Memory Usage**: ~150-200 MB (model + buffers)
- **CPU Usage**: Moderate (depends on Whisper model size)
- **Latency**: ~15 seconds (chunk processing time)
- **Accuracy**: Depends on Whisper model (base model provides good balance)

## Limitations

- Currently supports English only (can be extended to other languages)
- Processes audio in 15-second chunks (not truly real-time)
- Requires local Whisper model (no cloud API)
- Best results with clear audio and minimal background noise

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [OpenAI Whisper](https://github.com/openai/whisper) - The amazing speech recognition model
- [whisper-rs](https://github.com/tazz4843/whisper-rs) - Rust bindings for Whisper
- [cpal](https://github.com/RustAudio/cpal) - Cross-platform audio library
- [BlackHole](https://github.com/ExistentialAudio/BlackHole) - Virtual audio driver for macOS

## Support

- **Issues**: [GitHub Issues](https://github.com/vikashviraj/whyme/issues)
- **Discussions**: [GitHub Discussions](https://github.com/vikashviraj/whyme/discussions)

## Author

Created with ❤️ for the open-source community.

---

**Note**: This project is not affiliated with OpenAI. Whisper is used under OpenAI's terms of service.

