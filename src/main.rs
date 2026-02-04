use std::io;

mod app;
mod providers;
mod models;
mod ui;
mod theme;
mod user;
mod error;
mod config;

use app::App;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal).await;
    ratatui::restore();
    app_result
}