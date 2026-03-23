use foot_info_core::error::AppError;
use foot_info_core::models::{LeagueStats, Match, TopMatch};
use crate::handlers;
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
    FetchLeagueStats(String),
    LeagueStatsFound(LeagueStats),
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
                            let client = self.state.client.clone();
                            let provider = self.state.client.providers()[self.state.current_provider_index].country();
                            let team = team.clone();
                            tokio::spawn(async move {
                                match client.search_team(&team, provider).await {
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
                            let client = self.state.client.clone();
                            tokio::spawn(async move {
                                match client.fetch_top_matches().await {
                                    Ok(top_matches) => {
                                        let _ = tx.send(Action::TopMatchesFound(top_matches));
                                    }
                                    Err(e) => {
                                        let _ = tx.send(Action::Error(e));
                                    }
                                }
                            });
                        }
                        Action::FetchLeagueStats(ref url) => {
                            let tx = self.action_tx.clone();
                            let client = self.state.client.clone();
                            let url = url.clone();
                            tokio::spawn(async move {
                                match client.fetch_league_stats(&url).await {
                                    Ok(stats) => {
                                        let _ = tx.send(Action::LeagueStatsFound(stats));
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
