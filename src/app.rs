use std::io;
use std::sync::Arc;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;
use crate::ui;
use crate::models::Match;
use crate::providers::{FootballProvider, wheresthematch::WheresTheMatchProvider, mock_us::MockUsProvider};
use crate::error::AppError;
use crate::config::Config;

pub enum Action {
    Search(String),
    MatchesFound(Vec<Match>),
    Error(AppError),
}

pub struct App {
    pub search_input: String,
    pub matches: Vec<Match>,
    pub error_message: Option<String>,
    pub status_message: Option<String>,
    pub is_loading: bool,
    pub exit: bool,
    pub config: Config,
    pub providers: Vec<Arc<dyn FootballProvider>>,
    pub current_provider_index: usize,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
}

impl App {
    pub fn new() -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        let config = Config::load();
        Self {
            search_input: String::new(),
            matches: Vec::new(),
            error_message: None,
            status_message: None,
            is_loading: false,
            exit: false,
            config,
            providers: vec![
                Arc::new(WheresTheMatchProvider),
                Arc::new(MockUsProvider),
            ],
            current_provider_index: 0,
            action_tx,
            action_rx,
        }
    }

    pub fn get_current_provider(&self) -> Arc<dyn FootballProvider> {
        self.providers[self.current_provider_index].clone()
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
                        let provider = self.get_current_provider();
                        tokio::spawn(async move {
                            match provider.fetch_matches_channels(&team).await {
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
                    self.status_message = None;
                    let _ = self.action_tx.send(Action::Search(self.search_input.clone()));
                }
            }
            KeyCode::Char('s') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                if !self.search_input.is_empty() {
                    self.config.favorite_team = Some(self.search_input.clone());
                    if let Err(e) = self.config.save() {
                        self.error_message = Some(format!("Failed to save config: {}", e));
                    } else {
                        self.status_message = Some(format!("Saved favorite: {}", self.search_input));
                    }
                }
            }
            KeyCode::Char('f') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                if let Some(team) = &self.config.favorite_team {
                    self.search_input = team.clone();
                    self.status_message = Some(format!("Loaded favorite: {}", team));
                    let _ = self.action_tx.send(Action::Search(team.clone()));
                } else {
                    self.status_message = Some("No favorite team saved.".to_string());
                }
            }
            KeyCode::Char('c') => {
                self.current_provider_index = (self.current_provider_index + 1) % self.providers.len();
                let provider = self.get_current_provider();
                self.status_message = Some(format!("Switched to: {} ({})", provider.country(), provider.name()));
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