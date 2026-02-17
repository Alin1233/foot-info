use crate::error::AppError;
use crate::handlers;
use crate::models::{Match, TopMatch};
use crate::providers::livesoccertv;
use crate::state::AppState;
use crate::ui;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;
use tokio::sync::mpsc;

pub enum Action {
    Search(String),
    MatchesFound(Vec<Match>),
    Error(AppError),
    FetchTopMatches,
    TopMatchesFound(Vec<TopMatch>),
}

pub struct App {
    pub state: AppState,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
}

impl App {
    pub fn new() -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        Self {
            state: AppState::new(),
            action_tx,
            action_rx,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.state.exit {
            terminal.draw(|frame| ui::draw(frame, &self.state))?;

            // Handle terminal events
            if event::poll(std::time::Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        if let Some(action) = handlers::handle_key_event(&mut self.state, key_event)
                        {
                            let _ = self.action_tx.send(action);
                        }
                    }
                    _ => {}
                }
            }

            // Handle async actions
            while let Ok(action) = self.action_rx.try_recv() {
                let should_spawn = handlers::handle_action(&mut self.state, &action);

                if should_spawn {
                    match action {
                        Action::Search(ref team) => {
                            let tx = self.action_tx.clone();
                            let provider = self.state.get_current_provider();
                            let team = team.clone();
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
                        Action::FetchTopMatches => {
                            let tx = self.action_tx.clone();
                            tokio::spawn(async move {
                                match livesoccertv::fetch_top_matches().await {
                                    Ok(top_matches) => {
                                        let _ = tx.send(Action::TopMatchesFound(top_matches));
                                    }
                                    Err(e) => {
                                        let _ = tx.send(Action::Error(e));
                                    }
                                }
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}
