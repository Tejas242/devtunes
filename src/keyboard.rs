use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone)] // Add this line
pub struct Key {
    pub note: String,
    pub frequency: f32,
    pub is_pressed: bool,
    pub press_time: Option<Instant>,
}

pub struct Keyboard {
    pub keys: HashMap<char, Key>,
    pub active_keys: Vec<char>,
}

impl Keyboard {
    pub fn new() -> Self {
        let mut keys = HashMap::new();

        // Define keys with their notes and frequencies
        let key_mappings = vec![
            ('a', "A4", 440.0),
            ('s', "B4", 493.88),
            ('d', "C4", 523.25),
            ('f', "D4", 587.33),
            ('g', "E4", 659.25),
            ('h', "F4", 698.46),
            ('j', "G4", 783.99),
            ('k', "A5", 880.00),
            ('l', "B5", 987.77),
        ];

        for (key, note, freq) in key_mappings {
            keys.insert(
                key,
                Key {
                    note: note.to_string(),
                    frequency: freq,
                    is_pressed: false,
                    press_time: None,
                },
            );
        }

        Keyboard {
            keys,
            active_keys: Vec::new(),
        }
    }

    pub fn press_key(&mut self, key: char) {
        if let Some(k) = self.keys.get_mut(&key) {
            k.is_pressed = true;
            k.press_time = Some(Instant::now());
            if !self.active_keys.contains(&key) {
                self.active_keys.push(key);
            }
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.keys.iter_mut().for_each(|(_, key)| {
            if let Some(press_time) = key.press_time {
                if now.duration_since(press_time) > Duration::from_millis(150) {
                    key.is_pressed = false;
                    key.press_time = None;
                }
            }
        });
        self.active_keys.retain(|&k| self.keys[&k].is_pressed);
    }
}
