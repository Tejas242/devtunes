use crate::drums::DrumSound;
use crate::instrument::{CustomWaveform, InstrumentType};
use crate::recorder::Note;
use rodio::{OutputStream, Sink, Source};
use std::time::Duration;

pub struct AudioEngine {
    _stream: OutputStream,
    sink: Sink,
    current_instrument: InstrumentType,
}

impl AudioEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        Ok(AudioEngine {
            _stream: stream,
            sink,
            current_instrument: InstrumentType::Sine,
        })
    }

    pub fn change_instrument(&mut self, instrument: InstrumentType) {
        self.current_instrument = instrument;
    }

    fn get_note_frequency(key: char) -> f32 {
        match key.to_lowercase().next().unwrap() {
            'a' => 440.0,  // A4
            's' => 493.88, // B4
            'd' => 523.25, // C4
            'f' => 587.33, // D4
            'g' => 659.25, // E4
            'h' => 698.46, // F4
            'j' => 783.99, // G4
            'k' => 880.00, // A5
            'l' => 987.77, // B5
            _ => 440.0,    // Default to A4
        }
    }

    pub fn play_note(&self, key: char) {
        let frequency = Self::get_note_frequency(key);
        let source = CustomWaveform::new(frequency, self.current_instrument)
            .take_duration(Duration::from_millis(150))
            .amplify(0.20);

        self.sink.append(source);
    }

    pub fn play_chord(&self, keys: &[char]) {
        for &key in keys {
            let frequency = Self::get_note_frequency(key);
            let source = CustomWaveform::new(frequency, self.current_instrument)
                .take_duration(Duration::from_millis(150))
                .amplify(0.15); // Slightly lower amplitude for chords

            self.sink.append(source);
        }
    }

    pub fn play_recording(&self, recording: &[Note]) {
        for note in recording {
            self.play_note(note.key);
            std::thread::sleep(Duration::from_millis(150)); // Fixed delay between notes
        }
    }

    pub fn play_drum(&self, drum: DrumSound) {
        let source = CustomWaveform::new(drum.frequency(), self.current_instrument)
            .take_duration(Duration::from_millis(drum.duration()))
            .amplify(0.3);

        self.sink.append(source);
    }
}
