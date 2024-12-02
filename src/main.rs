mod audio;
mod drums;
mod instrument;
mod keyboard;
mod recorder;
mod ui;

use crossterm::event::{self, Event, KeyCode};
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = ui::App::new()?;
    let mut audio = audio::AudioEngine::new()?;
    let mut is_playing = false;

    loop {
        app.draw()?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => {
                        if app.recorder.is_recording {
                            app.recorder.stop_recording();
                        } else {
                            app.recorder.start_recording();
                        }
                    }
                    KeyCode::Char('p') => {
                        if !app.recorder.is_recording && !is_playing {
                            is_playing = true;
                            let recording = app.recorder.get_recording();
                            audio.play_recording(recording);
                            is_playing = false;
                        }
                    }
                    KeyCode::Char('1') => {
                        audio.change_instrument(instrument::InstrumentType::Sine);
                        app.set_instrument("Sine");
                    }
                    KeyCode::Char('2') => {
                        audio.change_instrument(instrument::InstrumentType::Square);
                        app.set_instrument("Square");
                    }
                    KeyCode::Char('3') => {
                        audio.change_instrument(instrument::InstrumentType::Triangle);
                        app.set_instrument("Triangle");
                    }
                    KeyCode::Char('4') => {
                        audio.change_instrument(instrument::InstrumentType::Saw);
                        app.set_instrument("Saw");
                    }

                    KeyCode::Tab => {
                        app.drum_pad.toggle_mode();
                    }
                    KeyCode::Char(c) => {
                        if app.drum_pad.is_drum_mode {
                            if let Some(drum) = app.drum_pad.hit_drum(c) {
                                audio.play_drum(drum);
                                app.drum_pad.active_beats.push(c);
                            }
                        } else {
                            app.keyboard.press_key(c);
                            audio.play_note(c);
                            app.log_keystroke();

                            if app.recorder.is_recording {
                                app.recorder.record_note(c);
                            }

                            if app.keyboard.active_keys.len() > 1 {
                                audio.play_chord(&app.keyboard.active_keys);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    app.cleanup()?;
    Ok(())
}
