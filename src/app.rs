use std::time::Instant;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::seq::IndexedRandom;
use ratatui::layout::Constraint;
use ratatui::style::Stylize;
use ratatui::{
    layout::{Alignment, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

#[derive(Debug)]
pub struct App {
    running: bool,
    words: Vec<String>,
    input: String,
    start_time: Option<Instant>,
    wpm_data: Vec<u32>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: false,
            words: Self::generate_text(),
            input: String::new(),
            start_time: None,
            wpm_data: Vec::new(),
        }
    }
}

impl App {
    fn generate_text() -> Vec<String> {
        let word_list = [
            "hello",
            "world",
            "rust",
            "speed",
            "test",
            "keyboard",
            "fast",
            "typing",
            "game",
            "challenge",
            "performance",
            "accuracy",
        ];
        let mut rng = rand::rng();
        word_list
            .choose_multiple(&mut rng, 10)
            .map(|s| s.to_string())
            .collect()
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events()?;
            self.update_wpm();
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .margin(3)
            .split(frame.area());

        let title = Line::from("Teaty Typing Speed Test")
            .bold()
            .blue()
            .centered();

        let mut text_display = String::new();
        for (i, word) in self.words.iter().enumerate() {
            if i > 0 {
                text_display.push(' ');
            }
            for (j, c) in word.chars().enumerate() {
                if let Some(input_char) = self.input.chars().nth(i * (word.len() + 1) + j) {
                    if input_char == c {
                        text_display.push_str(&c.to_string().green().to_string());
                    } else {
                        text_display.push_str(&c.to_string().red().to_string());
                    }
                } else {
                    text_display.push(c);
                }
            }
        }

        let wpm_display = format!("WPM: {}", self.wpm_data.last().unwrap_or(&0));

        let text_paragraph = Paragraph::new(text_display)
            .block(Block::bordered().title("Words to Type"))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightGreen));

        let wpm_paragraph = Paragraph::new(wpm_display)
            .block(Block::bordered().title("Speed (WPM)"))
            .alignment(Alignment::Left)
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_widget(
            Paragraph::new("").block(Block::bordered().title(title)),
            layout[0],
        );
        frame.render_widget(text_paragraph, layout[0]);
        frame.render_widget(wpm_paragraph, layout[0]);
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    // All keystrokes along with the exit logic implemented
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc)
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (KeyModifiers::NONE, KeyCode::Char(c)) => {
                if self.start_time.is_none() {
                    self.start_time = Some(Instant::now());
                }
                self.input.push(c);
            }
            (KeyModifiers::CONTROL, KeyCode::Char('r')) => self.reset(),
            _ => {}
        }
    }

    fn update_wpm(&mut self) {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed().as_secs();
            if elapsed > 0 {
                // Calculate words per minute (WPM)
                // WPM is calculated as the number of characters typed divided by 5 (average word length)
                // multiplied by 60 (seconds in a minute) divided by the elapsed time in seconds
                let wpm = (self.input.len() as f64 / 5.0) * (60.0 / elapsed as f64);

                // Store the calculated WPM in the wpm_data vector
                self.wpm_data.push(wpm as u32);
            }
        }
    }

    fn reset(&mut self) {
        self.words = Self::generate_text();
        self.input.clear();
        self.start_time = None;
        self.wpm_data.clear();
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
