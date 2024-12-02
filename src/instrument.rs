use rodio::Source;
use std::f32::consts::PI;
use std::time::Duration;

#[derive(Copy, Clone)]
pub enum InstrumentType {
    Sine,
    Square,
    Triangle,
    Saw,
}

pub struct CustomWaveform {
    frequency: f32,
    num_samples: usize,
    instrument: InstrumentType,
    sample_rate: u32,
    position: usize,
}

impl CustomWaveform {
    pub fn new(frequency: f32, instrument: InstrumentType) -> Self {
        Self {
            frequency,
            num_samples: 0,
            instrument,
            sample_rate: 44100,
            position: 0,
        }
    }

    fn get_sample(&self, position: usize) -> f32 {
        let t = position as f32 / self.sample_rate as f32;
        let x = t * self.frequency * 2.0 * PI;

        match self.instrument {
            InstrumentType::Sine => x.sin(),
            InstrumentType::Square => {
                if x.sin() >= 0.0 {
                    0.5
                } else {
                    -0.5
                }
            }
            InstrumentType::Triangle => (2.0 * (x / (2.0 * PI)).fract() - 1.0).abs() * 2.0 - 1.0,
            InstrumentType::Saw => (x / (2.0 * PI)).fract() * 2.0 - 1.0,
        }
    }
}

impl Iterator for CustomWaveform {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.position = self.position.wrapping_add(1);
        Some(self.get_sample(self.position))
    }
}

impl Source for CustomWaveform {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
