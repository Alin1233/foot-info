use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;
use crate::ui;
use crate::models::Match;
use crate::client;
use crate::error::AppError;

#[derive(Debug)]
pub enum Action {
    Search(String),
    MatchesFound(Vec<Match>),
    Error(AppError),
}

#[derive(Debug)]
pub struct App {
    pub search_input: String,
    pub matches: Vec<Match>,
    pub error_message: Option<String>,
    pub is_loading: bool,
    pub exit: bool,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
}

impl App {
    pub fn new() -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        Self {
            search_input: String::new(),
            matches: Vec::new(),
            error_message: None,
            is_loading: false,
            exit: false,
            action_tx,
            action_rx,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| ui::draw(frame, self))?;

            // Handle terminal events
            if event::poll(std::time::Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        self.handle_key_event(key_event);
                    }
                    _ => {}
                }
            }
            // Handle async actions
            while let Ok(action) = self.action_rx.try_recv() {
                match action {
                    Action::Search(team) => {
                        self.is_loading = true;
                        self.error_message = None;
                        self.matches.clear();
                        let tx = self.action_tx.clone();
                        tokio::spawn(async move {
                            match client::fetch_matches(&team).await {
                                Ok(matches) => {
                                    let _ = tx.send(Action::MatchesFound(matches));
                                }
                                Err(e) => {
                                    let _ = tx.send(Action::Error(e));
                                }
                            }
                        });
                    }
                    Action::MatchesFound(matches) => {
                        self.is_loading = false;
                        self.matches = matches;
                    }
                    Action::Error(e) => {
                        self.is_loading = false;
                        self.error_message = Some(e.to_string());
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit = true,
            KeyCode::Enter => {
                if !self.search_input.is_empty() {
                    let _ = self.action_tx.send(Action::Search(self.search_input.clone()));
                }
            }
            KeyCode::Char(c) => {
                self.search_input.push(c);
            }
            KeyCode::Backspace => {
                self.search_input.pop();
            }
            _ => {}
        }
    }
}