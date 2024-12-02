use crate::drums::DrumPad;
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
}

impl App {
    pub fn new() -> Result<App, std::io::Error> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(App {
            terminal,
            keystroke_count: 0,
            current_instrument: "Sine".to_string(),
            keyboard: Keyboard::new(),
            recorder: Recorder::new(),
            drum_pad: DrumPad::new(),
        })
    }

    pub fn draw(&mut self) -> Result<(), std::io::Error> {
        self.keyboard.update();

        let current_instrument = self.current_instrument.clone();
        let keystroke_count = self.keystroke_count;
        let is_recording = self.recorder.is_recording;
        let is_drum_mode = self.drum_pad.is_drum_mode;
        let keyboard_keys = self.keyboard.keys.clone();
        let active_beats = self.drum_pad.active_beats.clone();

        self.terminal.draw(|frame| {
            let size = frame.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Title bar
                    Constraint::Length(2), // Mode tabs
                    Constraint::Length(5), // Info bar
                    Constraint::Min(10),   // Main content
                    Constraint::Length(3), // Status bar
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

            // Info bar
            let info_text = vec![
                Line::from(vec![
                    Span::styled("Instrument: ", INACTIVE_STYLE),
                    Span::styled(&current_instrument, HIGHLIGHT_STYLE),
                ]),
                Line::from(vec![
                    Span::styled("Keystrokes: ", INACTIVE_STYLE),
                    Span::styled(keystroke_count.to_string(), HIGHLIGHT_STYLE),
                ]),
                Line::from(vec![
                    Span::styled("Status: ", INACTIVE_STYLE),
                    if is_recording {
                        Span::styled("⏺ Recording", ERROR_STYLE)
                    } else {
                        Span::styled("◯ Ready", ACTIVE_STYLE)
                    },
                ]),
            ];
            let info = Paragraph::new(info_text).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(INACTIVE_STYLE),
            );
            frame.render_widget(info, chunks[2]);

            // Main content area
            let main_area = chunks[3];
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

                    let style = if active_beats.contains(key) {
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
                ("1-4", "Change Sound"),
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
            frame.render_widget(status, chunks[4]);
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
