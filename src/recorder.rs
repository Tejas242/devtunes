use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Note {
    pub key: char,
    pub timestamp: Duration,
}

pub struct Recorder {
    pub recording: Vec<Note>,
    pub is_recording: bool,
    start_time: Option<Instant>,
}

impl Recorder {
    pub fn new() -> Self {
        Self {
            recording: Vec::new(),
            is_recording: false,
            start_time: None,
        }
    }

    pub fn start_recording(&mut self) {
        self.recording.clear();
        self.is_recording = true;
        self.start_time = Some(Instant::now());
    }

    pub fn stop_recording(&mut self) {
        self.is_recording = false;
        self.start_time = None;
    }

    pub fn record_note(&mut self, key: char) {
        if let Some(start) = self.start_time {
            let timestamp = start.elapsed();
            self.recording.push(Note { key, timestamp });
        }
    }

    pub fn get_recording(&self) -> &Vec<Note> {
        &self.recording
    }
}
