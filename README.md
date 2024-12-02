# 🎵 DevTunes

DevTunes is an AI-powered terminal-based music creation tool written in Rust. Create music, play beats, and let AI generate melodies for you - all from your terminal!

![Devtunes screenshot](devtunes.png)
## ✨ Features

- 🎹 Musical keyboard with multiple waveforms:
  - 9 notes (A-L keys) mapped to standard frequencies
  - Sine, Square, Triangle, and Saw waveforms
  - Chord support for multiple key presses

- 🤖 AI-Powered Music Generation:
  - Uses Google's Gemini AI to generate melodies
  - Multiple mood options (Happy, Melancholic, Energetic, Calm)
  - Real-time melody playback

- 🥁 Drum Machine:
  - 5 different drum sounds (Kick, Snare, Hi-hat, Clap, Cymbal)
  - Toggle between keyboard and drum modes
  - Visual feedback for beats

- 🎼 Recording Features:
  - Record and playback your compositions
  - Save multiple patterns
  - Real-time visualization

- 👾 Terminal UI:
  - Beautiful, responsive interface using Ratatui
  - Visual keyboard/drum pad feedback
  - Status indicators and controls
  - AI status display

## 🚀 Quick Start

### Prerequisites

- Rust toolchain ([install from rustup.rs](https://rustup.rs/))
- Google Gemini API key for AI features
- Audio development files (Linux only)

#### Linux Dependencies

Ubuntu/Debian:
```bash
sudo apt-get install libasound2-dev
```

Fedora:
```bash
sudo dnf install alsa-lib-devel
```

### Installation

```bash
# Clone the repository
git clone https://github.com/tejas242/devtunes.git

# Enter directory
cd devtunes

# Set your Gemini API key (optional, for AI features)
export GEMINI_API_KEY=your_api_key_here

# Run the application
cargo run
```

## 🎮 Controls

### Global Controls
- `Q` - Quit application
- `TAB` - Switch between keyboard and drum modes
- `M` - Toggle AI mode
- `R` - Start/stop recording
- `P` - Play recorded sequence

### Keyboard Mode
- `1-4` - Change waveform (when AI mode off):
  - `1` - Sine wave
  - `2` - Square wave
  - `3` - Triangle wave
  - `4` - Saw wave

### AI Mode
- `1-4` - Change melody mood:
  - `1` - Happy
  - `2` - Melancholic
  - `3` - Energetic
  - `4` - Calm

### Musical Notes (Keyboard Mode)
```
A - A4 (440.00 Hz)
S - B4 (493.88 Hz)
D - C4 (523.25 Hz)
F - D4 (587.33 Hz)
G - E4 (659.25 Hz)
H - F4 (698.46 Hz)
J - G4 (783.99 Hz)
K - A5 (880.00 Hz)
L - B5 (987.77 Hz)
```

### Drum Mode
- `Z` - Kick drum
- `X` - Snare
- `C` - Hi-hat
- `V` - Clap
- `B` - Cymbal

## 🧪 Development

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Build release version
cargo build --release
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📚 Technologies

- [Ratatui](https://github.com/tui-rs-revival/ratatui) - Terminal interface
- [rodio](https://github.com/RustAudio/rodio) - Audio playback
- [Google Generative AI](https://github.com/avastmick/google-generative-ai-rs) - AI melody generation
- [Tokio](https://tokio.rs/) - Async runtime
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Made with ❤️ using 🦀 Rust
