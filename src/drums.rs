use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum DrumSound {
    Kick,
    Snare,
    HiHat,
    Clap,
    Cymbal,
}

impl DrumSound {
    pub fn frequency(&self) -> f32 {
        match self {
            DrumSound::Kick => 60.0,    // Low frequency for kick
            DrumSound::Snare => 200.0,  // Mid frequency for snare
            DrumSound::HiHat => 1000.0, // High frequency for hi-hat
            DrumSound::Clap => 300.0,   // Mid-high for clap
            DrumSound::Cymbal => 800.0, // High for cymbal
        }
    }

    pub fn duration(&self) -> u64 {
        match self {
            DrumSound::Kick => 150,
            DrumSound::Snare => 100,
            DrumSound::HiHat => 50,
            DrumSound::Clap => 80,
            DrumSound::Cymbal => 200,
        }
    }
}

pub struct DrumPad {
    pub sounds: HashMap<char, DrumSound>,
    pub is_drum_mode: bool,
    pub active_beats: Vec<char>,
}

impl DrumPad {
    pub fn new() -> Self {
        let mut sounds = HashMap::new();
        sounds.insert('z', DrumSound::Kick);
        sounds.insert('x', DrumSound::Snare);
        sounds.insert('c', DrumSound::HiHat);
        sounds.insert('v', DrumSound::Clap);
        sounds.insert('b', DrumSound::Cymbal);

        Self {
            sounds,
            is_drum_mode: false,
            active_beats: Vec::new(),
        }
    }

    pub fn toggle_mode(&mut self) {
        self.is_drum_mode = !self.is_drum_mode;
        self.active_beats.clear();
    }

    pub fn hit_drum(&mut self, key: char) -> Option<DrumSound> {
        if self.is_drum_mode {
            self.sounds.get(&key).copied()
        } else {
            None
        }
    }
}
