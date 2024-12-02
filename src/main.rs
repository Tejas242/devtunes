mod audio;
mod drums;
mod gemini_player;
mod instrument;
mod keyboard;
mod recorder;
mod ui;

use crossterm::event::{self, Event, KeyCode};
use ratatui::style::{Color, Modifier, Style};
use std::error::Error;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = ui::App::new()?;
    let mut audio = audio::AudioEngine::new()?;
    let mut is_playing = false;

    loop {
        // Release any keys that have been pressed long enough
        app.keyboard.release_keys();

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
                    KeyCode::Char('m') => {
                        app.ai_mode = !app.ai_mode;
                        if app.ai_mode {
                            app.start_ai_loading("Initializing AI Mode");
                            if let Some(player) = &mut app.gemini_player {
                                match player.generate_melody(&app.ai_mood).await {
                                    Ok(_) => {
                                        app.ai_last_generate = Some(Instant::now());
                                        app.finish_ai_loading(true, "AI Mode Ready");
                                        app.set_ai_response(
                                            "Initial melody generated successfully".to_string(),
                                        );
                                    }
                                    Err(e) => {
                                        app.finish_ai_loading(false, "AI Initialization Failed");
                                        app.set_ai_response(format!("Error: {}", e));
                                        app.ai_mode = false;
                                    }
                                }
                            } else {
                                app.finish_ai_loading(false, "No API Key Found");
                                app.ai_mode = false;
                            }
                        } else {
                            app.set_ai_status(None);
                            app.ai_response = None;
                        }
                    }

                    KeyCode::Char(n @ '1'..='4') if app.ai_mode => {
                        let mood = match n {
                            '1' => "happy",
                            '2' => "melancholic",
                            '3' => "energetic",
                            '4' => "calm",
                            _ => unreachable!(),
                        };
                        app.ai_mood = mood.to_string();
                        app.start_ai_loading(&format!("Generating {} melody", mood));

                        if let Some(player) = &mut app.gemini_player {
                            match player.generate_melody(mood).await {
                                Ok(_) => {
                                    app.ai_last_generate = Some(Instant::now());
                                    app.finish_ai_loading(true, &format!("{} melody ready", mood));
                                    app.set_ai_response(format!(
                                        "Generated new {} melody pattern",
                                        mood
                                    ));
                                }
                                Err(e) => {
                                    app.finish_ai_loading(false, "Generation Failed");
                                    app.set_ai_response(format!("Error: {}", e));
                                }
                            }
                        }
                    }
                    KeyCode::Char('1') if !app.ai_mode => {
                        audio.change_instrument(instrument::InstrumentType::Sine);
                        app.set_instrument("Sine");
                    }
                    KeyCode::Char('2') if !app.ai_mode => {
                        audio.change_instrument(instrument::InstrumentType::Square);
                        app.set_instrument("Square");
                    }
                    KeyCode::Char('3') if !app.ai_mode => {
                        audio.change_instrument(instrument::InstrumentType::Triangle);
                        app.set_instrument("Triangle");
                    }
                    KeyCode::Char('4') if !app.ai_mode => {
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

        // Handle AI-generated notes
        if app.ai_mode {
            if let Some(player) = &mut app.gemini_player {
                if let Some((note, duration)) = player.get_next_note() {
                    // Update keyboard state to show the pressed key
                    app.keyboard.press_key(note);

                    // Play the note
                    audio.play_note(note);

                    // Wait for the duration
                    std::thread::sleep(duration);

                    // Optional: Update keystroke count for AI-generated notes too
                    app.log_keystroke();
                }
            }
        }
    }

    app.cleanup()?;
    Ok(())
}
