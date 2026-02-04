# Project Architecture: Football Match Scraper

This document outlines the architecture and resources used in the **foot-info** TUI application.

## Overview
**foot-info** is a Terminal User Interface (TUI) application written in Rust. It allows users to search for a football team and scrapes match schedules and TV broadcast information from various sources based on the selected country.

## Architecture
The application follows a **Provider-based Architecture**, allowing for easy extension of data sources.

### 1. **Entry Point (`src/main.rs`)**
- Initializes the Tokio async runtime (`#[tokio::main]`).
- Sets up the `Ratatui` terminal backend.
- Launches the main application loop (`App::run`).

### 2. **Application State (`src/app.rs`)**
- **Responsibility**: Manages the runtime state of the application.
- **Key Components**:
  - `search_input`: Stores the user's current query.
  - `matches`: A vector of `Match` structs containing the scraped data.
  - `providers`: A list of available data providers (`Vec<Arc<dyn FootballProvider>>`).
  - `current_provider_index`: Tracks the currently active provider.
  - `config`: Handles user preferences (loaded from `src/config.rs`).
  - **Async Event Loop**: Uses `tokio::sync::mpsc` channels to handle asynchronous scraping tasks without freezing the UI.

### 3. **The Provider System (`src/providers/`)**
- **Pattern**: Strategy Pattern via the `FootballProvider` trait.
- **Trait Definition (`src/providers/mod.rs`)**:
  - `fetch_matches_channels(&self, team: &str)`: Async method to get data.
  - `country(&self)`: Returns the `Country` enum (UK, US, FR).
- **Implementations**:
  - **`WheresTheMatchProvider`** (UK): Scrapes [WherestheMatch.com](https://www.wheresthematch.com).
  - **`WorldSoccerTalkProvider`** (US): Scrapes [WorldSoccerTalk.com](https://worldsoccertalk.com).
  - **`MatchsTvProvider`** (FR): Scrapes [Matchs.tv](https://matchs.tv).

### 4. **UI Layer (`src/ui.rs` & `src/theme.rs`)**
- **Responsibility**: Renders the application to the terminal frame.
- **Features**:
  - Displays the current country context (e.g., `[UK]`, `[US]`).
  - Shows instructions for shortcuts:
    - `<Ctrl+s>`: Save current team as favorite.
    - `<Ctrl+f>`: Load favorite team.
    - `<c>`: Switch country/provider.

### 5. **Data Layer (`src/models.rs`)**
- `Match`: Holds `teams`, `competition`, `date`, `time`, and `channels`.
- `Country`: Enum (`UK`, `US`, `FR`) representing the region of the provider.

### 6. **Utilities**
- **`src/user.rs`**: Handles timezone conversions.
  - `convert_utc_to_local`: For ISO 8601 timestamps.
  - `convert_et_to_local`: For US Eastern Time (WorldSoccerTalk).
  - `convert_french_time_to_local`: For Paris Time (Matchs.tv).
- **`src/config.rs`**: Manages persistence of user preferences (favorite team) using `serde` and the system's config directory.

## Resources & Libraries

| Library | Purpose |
| :--- | :--- |
| **Ratatui** | The core TUI framework for rendering the interface. |
| **Tokio** | Asynchronous runtime for non-blocking network requests. |
| **Crossterm** | Handles low-level terminal input/output events. |
| **Reqwest** | HTTP client for fetching HTML pages. |
| **Scraper** | HTML parsing library using CSS selectors. |
| **Chrono** | Date and time manipulation. |
| **Chrono-TZ** | Timezone database for converting ET/Paris times to local. |
| **Serde** | Serialization for configuration files. |
| **Async-Trait** | Enables async methods in the `FootballProvider` trait. |

## External Data Sources
- **UK**: [WherestheMatch.com](https://www.wheresthematch.com)
- **US**: [WorldSoccerTalk.com](https://worldsoccertalk.com)
- **FR**: [Matchs.tv](https://matchs.tv)
