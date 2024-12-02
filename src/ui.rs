use crate::drums::DrumPad;
use crate::gemini_player::GeminiPlayer;
use crate::keyboard::Keyboard;
use crate::recorder::Recorder;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Terminal,
};
use std::io::stdout;
use std::time::Duration;
use std::time::Instant;

const TITLE_STYLE: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
const HIGHLIGHT_STYLE: Style = Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD);
const ACTIVE_STYLE: Style = Style::new().fg(Color::Green).add_modifier(Modifier::BOLD);
const INACTIVE_STYLE: Style = Style::new().fg(Color::Gray);
const ERROR_STYLE: Style = Style::new().fg(Color::Red).add_modifier(Modifier::BOLD);

pub struct App {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    keystroke_count: u32,
    current_instrument: String,
    pub keyboard: Keyboard,
    pub recorder: Recorder,
    pub drum_pad: DrumPad,
    pub gemini_player: Option<GeminiPlayer>,
    pub ai_mode: bool,
    pub ai_mood: String,
    pub ai_last_generate: Option<Instant>,
    pub ai_status: Option<(String, Style)>,
    pub ai_loading: bool,
    pub ai_response: Option<String>,
    pub ai_response_time: Option<Instant>,
}

impl App {
    pub fn new() -> Result<App, std::io::Error> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        // Initialize GeminiPlayer if API key is available
        let gemini_player = std::env::var("GEMINI_API_KEY")
            .map(|key| Some(GeminiPlayer::new(&key)))
            .unwrap_or(None);

        Ok(App {
            terminal,
            keystroke_count: 0,
            current_instrument: "Sine".to_string(),
            keyboard: Keyboard::new(),
            recorder: Recorder::new(),
            drum_pad: DrumPad::new(),
            gemini_player,
            ai_mode: false,
            ai_mood: "happy".to_string(),
            ai_last_generate: None,
            ai_status: None,
            ai_loading: false,
            ai_response: None,
            ai_response_time: None,
        })
    }

    pub fn set_ai_status(&mut self, message: Option<(String, Style)>) {
        self.ai_status = message;
    }

    pub fn start_ai_loading(&mut self, action: &str) {
        self.ai_loading = true;
        self.set_ai_status(Some((
            format!("⟳ {} ...", action),
            Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )));
    }

    pub fn set_ai_response(&mut self, response: String) {
        self.ai_response = Some(response);
        self.ai_response_time = Some(Instant::now());
    }

    pub fn clear_ai_response(&mut self) {
        if let Some(time) = self.ai_response_time {
            if time.elapsed() > Duration::from_secs(5) {
                self.ai_response = None;
                self.ai_response_time = None;
            }
        }
    }

    pub fn finish_ai_loading(&mut self, success: bool, message: &str) {
        self.ai_loading = false;
        self.set_ai_status(Some((
            format!("{} {}", if success { "✓" } else { "✗" }, message),
            if success {
                Style::new().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::new().fg(Color::Red).add_modifier(Modifier::BOLD)
            },
        )));
    }

    pub fn draw(&mut self) -> Result<(), std::io::Error> {
        self.keyboard.update();
        self.clear_ai_response();

        let current_instrument = self.current_instrument.clone();
        let keystroke_count = self.keystroke_count;
        let is_recording = self.recorder.is_recording;
        let is_drum_mode = self.drum_pad.is_drum_mode;
        let keyboard_keys = self.keyboard.keys.clone();
        let active_beats = self.drum_pad.active_beats.clone();

        self.terminal.draw(|frame| {
            let size = frame.size();

            // Modified layout to include AI status area
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Title bar [0]
                    Constraint::Length(2), // Mode tabs [1]
                    Constraint::Length(7), // Info bar [2]
                    Constraint::Length(4), // AI Response area [3]
                    Constraint::Min(12),   // Main content [4]
                    Constraint::Length(3), // Status bar [5]
                ])
                .split(size);

            // Title bar
            let title = format!(
                "{}DevTunes v0.1.0{}",
                symbols::line::DOUBLE_VERTICAL,
                symbols::line::DOUBLE_VERTICAL
            );
            let title_widget = Paragraph::new(title)
                .style(TITLE_STYLE)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(title_widget, chunks[0]);

            // Mode tabs
            let titles = vec!["Keyboard Mode", "Drum Mode"];
            let tabs = Tabs::new(titles)
                .select(if is_drum_mode { 1 } else { 0 })
                .style(INACTIVE_STYLE)
                .highlight_style(HIGHLIGHT_STYLE)
                .divider(symbols::line::VERTICAL);
            frame.render_widget(tabs, chunks[1]);

            // Info bar with AI status
            let mut info_text = vec![
                Line::from(vec![
                    Span::styled("Instrument: ", INACTIVE_STYLE),
                    Span::styled(&current_instrument, HIGHLIGHT_STYLE),
                ]),
                Line::from(vec![
                    Span::styled("Status: ", INACTIVE_STYLE),
                    if is_recording {
                        Span::styled("⏺ Recording", ERROR_STYLE)
                    } else {
                        Span::styled("◯ Ready", ACTIVE_STYLE)
                    },
                ]),
                Line::from(vec![
                    Span::styled("AI Mode: ", INACTIVE_STYLE),
                    if self.ai_mode {
                        if self.ai_loading {
                            Span::styled("⟳ Processing", Style::new().fg(Color::Yellow))
                        } else {
                            Span::styled(format!("ON - {} Mood", self.ai_mood), ACTIVE_STYLE)
                        }
                    } else {
                        Span::styled("OFF", INACTIVE_STYLE)
                    },
                ]),
            ];

            // Add AI status message if present
            if let Some((message, style)) = &self.ai_status {
                info_text.push(Line::from(Span::styled(message, *style)));
            }

            let info = Paragraph::new(info_text)
                .block(Block::default().borders(Borders::ALL).title("Status"))
                .wrap(ratatui::widgets::Wrap { trim: true });
            frame.render_widget(info, chunks[2]);

            // AI Response Area
            if self.ai_mode {
                let response_text = if self.ai_loading {
                    vec![Line::from(vec![
                        Span::styled("⟳ ", Style::new().fg(Color::Yellow)),
                        Span::styled("Generating melody...", Style::new().fg(Color::Yellow)),
                    ])]
                } else if let Some(response) = &self.ai_response {
                    vec![Line::from(vec![
                        Span::styled("Last Response: ", INACTIVE_STYLE),
                        Span::styled(response, HIGHLIGHT_STYLE),
                    ])]
                } else {
                    vec![Line::from(Span::styled(
                        "Waiting for AI generation...",
                        INACTIVE_STYLE,
                    ))]
                };

                let response_widget = Paragraph::new(response_text)
                    .block(Block::default().borders(Borders::ALL).title("AI Response"))
                    .wrap(ratatui::widgets::Wrap { trim: true });
                frame.render_widget(response_widget, chunks[3]);
            }

            // Main content area
            let main_area = chunks[4];
            if is_drum_mode {
                // Render drum pads
                let drums = vec![
                    ('z', "KICK", "💥"),
                    ('x', "SNARE", "🥁"),
                    ('c', "HIHAT", "🎪"),
                    ('v', "CLAP", "👏"),
                    ('b', "CYMBAL", "🔊"),
                ];

                let pad_width = main_area.width / 5;
                let pad_height = main_area.height / 2;

                for (i, (key, name, symbol)) in drums.iter().enumerate() {
                    let x = (i as u16 * pad_width) + 1;
                    let y = main_area.height - pad_height - 1;

                    let style = if active_beats.contains(&key) {
                        ACTIVE_STYLE
                    } else {
                        INACTIVE_STYLE
                    };

                    let pad_block = Block::default()
                        .borders(Borders::ALL)
                        .style(style)
                        .title(format!("{} {}", symbol, name));

                    let pad_area =
                        Rect::new(main_area.x + x, main_area.y + y, pad_width - 1, pad_height);
                    frame.render_widget(pad_block, pad_area);
                }
            } else {
                // Render keyboard
                let keys = vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];
                let key_width = main_area.width / 9;
                let key_height = main_area.height / 2;

                for (i, &key) in keys.iter().enumerate() {
                    let key_info = &keyboard_keys[&key];
                    let x = (i as u16 * key_width) + 1;
                    let y = main_area.height - key_height - 1;

                    let style = if key_info.is_pressed {
                        ACTIVE_STYLE
                    } else {
                        INACTIVE_STYLE
                    };

                    let key_block = Block::default()
                        .borders(Borders::ALL)
                        .style(style)
                        .title(format!("{} {}", key_info.note, key.to_uppercase()));

                    let key_area =
                        Rect::new(main_area.x + x, main_area.y + y, key_width - 1, key_height);
                    frame.render_widget(key_block, key_area);
                }
            }

            // Status bar
            let controls = vec![
                ("TAB", "Switch Mode"),
                (
                    "1-4",
                    if self.ai_mode {
                        "Change Mood"
                    } else {
                        "Change Sound"
                    },
                ),
                ("M", "AI Mode"),
                ("R", "Record"),
                ("P", "Play"),
                ("Q", "Quit"),
            ];
            let status_text = controls
                .into_iter()
                .map(|(key, action)| {
                    format!(
                        "{} {} {} {}",
                        symbols::line::VERTICAL,
                        key,
                        symbols::line::VERTICAL_LEFT,
                        action
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");

            let status = Paragraph::new(status_text)
                .style(INACTIVE_STYLE)
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(status, chunks[5]);
        })?;

        Ok(())
    }

    pub fn set_instrument(&mut self, instrument: &str) {
        self.current_instrument = instrument.to_string();
    }

    pub fn log_keystroke(&mut self) {
        self.keystroke_count += 1;
    }

    pub fn cleanup(&mut self) -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}
