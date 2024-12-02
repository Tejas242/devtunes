use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Loop {
    pub notes: Vec<(char, Duration)>,
    pub instrument: String,
    pub is_active: bool,
}

pub struct Looper {
    pub loops: Vec<Loop>,
    pub is_recording: bool,
    pub current_loop: usize,
    start_time: Option<Instant>,
    pub max_loops: usize,
}

impl Looper {
    pub fn new() -> Self {
        Self {
            loops: Vec::new(),
            is_recording: false,
            current_loop: 0,
            start_time: None,
            max_loops: 4, // Maximum 4 loops
        }
    }

    pub fn start_recording(&mut self) {
        self.is_recording = true;
        self.start_time = Some(Instant::now());

        // Create new loop if needed
        if self.current_loop >= self.loops.len() {
            self.loops.push(Loop {
                notes: Vec::new(),
                instrument: String::from("Default"),
                is_active: true,
            });
        }
    }

    pub fn stop_recording(&mut self) {
        self.is_recording = false;
        self.start_time = None;
        if self.current_loop < self.max_loops - 1 {
            self.current_loop += 1;
        }
    }

    pub fn record_note(&mut self, key: char, instrument: &str) {
        if let Some(start) = self.start_time {
            if let Some(loop_track) = self.loops.get_mut(self.current_loop) {
                loop_track.instrument = instrument.to_string();
                loop_track.notes.push((key, start.elapsed()));
            }
        }
    }

    pub fn toggle_loop(&mut self, index: usize) {
        if let Some(loop_track) = self.loops.get_mut(index) {
            loop_track.is_active = !loop_track.is_active;
        }
    }

    pub fn clear_loop(&mut self, index: usize) {
        if index < self.loops.len() {
            self.loops.remove(index);
            if self.current_loop >= self.loops.len() {
                self.current_loop = self.loops.len().saturating_sub(1);
            }
        }
    }
}
