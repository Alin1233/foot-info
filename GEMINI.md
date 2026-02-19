# Project Architecture: Football Match Scraper

This document outlines the architecture and resources used in the **foot-info** TUI application.

## Overview
**foot-info** is a Terminal User Interface (TUI) application written in Rust. It allows users to search for a football team and scrapes match schedules and TV broadcast information from various sources based on the selected country.

## Architecture
The application follows a **Provider-based Architecture** with clear separation of concerns across state, event handling, UI, and data layers.

### 1. **Entry Point (`src/main.rs`)**
- Initializes the Tokio async runtime (`#[tokio::main]`).
- Sets up the `Ratatui` terminal backend.
- Imports `App` from the library crate and launches `App::run`.

### 2. **Library Crate (`src/lib.rs`)**
- Exposes all modules as a public library crate, enabling integration tests in `tests/` to access internal APIs like `providers::wheresthematch::parse_html`.

### 3. **Application Layer**
Split into three focused modules:

#### **State (`src/state.rs`)**
- **Responsibility**: Pure application data — no channels, async, or side effects.
- **`AppState` struct** holds:
  - `search_input`, `matches`, `error_message`, `status_message`
  - `is_loading`, `exit`
  - `providers: Vec<Arc<dyn FootballProvider>>`, `current_provider_index`
  - `config: Config`
- Helper: `get_current_provider()` returns the active provider.

#### **Event Handling (`src/handlers/`)**
- **Responsibility**: Pure state mutations, easily testable without a terminal or runtime.
- **`mod.rs`**: Dispatcher — checks global shortcuts first, then delegates to mode-specific handler. Also owns `handle_action` for applying async results.
- **`search.rs`**: Handles Search-mode keybindings (Esc, Enter, Ctrl+s/f/t, char input).
- **`top_matches.rs`**: Handles TopMatches-mode keybindings (↑/↓/←/→ column navigation, Enter, Esc). Includes `date_groups`/`flat_to_col_row` helpers for column-based navigation.

#### **Orchestrator (`src/app.rs`)**
- **Responsibility**: Thin runtime shell — owns `AppState` + `mpsc` channels + the async run loop.
- `App::run()` coordinates: polls terminal events → delegates to `handlers` → spawns async tasks → processes results.
- `Action` enum: `Search(String)`, `MatchesFound(Vec<Match>)`, `Error(AppError)`, `FetchTopMatches`, `TopMatchesFound(Vec<TopMatch>)`.

### 4. **The Provider System (`src/providers/`)**
- **Pattern**: Strategy Pattern via the `FootballProvider` trait (with `#[cfg_attr(test, mockall::automock)]` for test mocking).
- **Trait Definition (`src/providers/mod.rs`)**:
  - `fetch_matches_channels(&self, team: &str)`: Async method to fetch and parse data.
  - `country(&self)`: Returns the `Country` enum (UK, US, FR).
  - `name(&self)`: Returns the provider's display name.
- **Implementations** (each exposes a `pub fn parse_html` for testability):
  - **`WheresTheMatchProvider`** (UK): Scrapes [WherestheMatch.com](https://www.wheresthematch.com). Uses `wreq` with Chrome 136 emulation to bypass TLS fingerprinting.
  - **`WorldSoccerTalkProvider`** (US): Scrapes [WorldSoccerTalk.com](https://worldsoccertalk.com). Uses `wreq` with Chrome 136 emulation.
  - **`MatchsTvProvider`** (FR): Scrapes [Matchs.tv](https://matchs.tv). Uses `wreq` with Chrome 136 emulation. Also exposes `pub fn parse_french_date` and `pub fn convert_french_time_to_local`.
- **Standalone Module** (does **not** implement `FootballProvider` — different purpose):
  - **`livesoccertv`**: Scrapes [LiveSoccerTV.com](https://www.livesoccertv.com/schedules/) "Upcoming Top Matches" section. Returns `Vec<TopMatch>`. Uses `wreq` with Chrome 136 emulation to bypass Cloudflare protection.

### 5. **UI Layer (`src/ui/`)**
Modular component-based structure:

```
src/ui/
├── mod.rs              # Module root, re-exports draw()
├── render.rs           # Thin dispatcher: outer frame + delegates to views
├── layout.rs           # Reusable layout functions (main_vertical, input_horizontal, results_horizontal)
├── theme.rs            # Color constants (BG_BLACK, GOLD, RUST_ORANGE, BEIGE)
├── views/
│   ├── mod.rs
│   ├── search.rs            # Search view composition (search bar + status + match list)
│   └── top_matches.rs       # Top matches view composition (status + date-grouped columns)
└── components/
    ├── mod.rs
    ├── search_bar.rs        # Search input widget
    ├── match_list.rs        # Results display (uses ResultsState enum: Loading/Error/Matches/Empty)
    ├── status_bar.rs        # Transient status messages
    └── top_matches_list.rs  # Upcoming top matches list with selection highlight (TopMatchesState enum)
```

- `render::draw(frame, &AppState)` is the entry point, rendering based on `ViewMode`.
- **Search mode shortcuts**: `<Esc>` quit, `<Enter>` search, `<Ctrl+s>` save favorite, `<Ctrl+f>` load favorite, `<Ctrl+c>` switch country, `<Ctrl+t>` top matches.
- **Top Matches mode shortcuts**: `<Esc>` back, `<↑/↓>` navigate, `<Enter>` select match, `<Ctrl+c>` switch country.

### 6. **Data Layer (`src/models.rs`)**
- `Match`: Holds `teams`, `competition`, `date`, `time`, and `channels`.
- `TopMatch`: Lightweight struct for discovery matches (`teams`, `date`, `match_url`).
- `ViewMode`: Enum (`Search`, `TopMatches`) tracking which screen is active.
- `Country`: Enum (`UK`, `US`, `FR`) representing the region of the provider.

### 7. **Error Handling (`src/error.rs`)**
- `AppError` enum with variants: `NetworkError`, `TeamNotFound`, `NoMatchesScheduled`.

### 8. **Utilities**
- **`src/user.rs`**: Handles timezone conversions.
  - `convert_utc_to_local`: For ISO 8601 timestamps (WheresTheMatch).
  - `convert_et_to_local`: For US Eastern Time (WorldSoccerTalk).
- **`src/config.rs`**: Manages persistence of user preferences (favorite team) using `serde` and the system's config directory.

## Testing

### Integration Tests (`tests/`)
Tests use **real HTML** fetched from live provider websites, stored in `tests/resources/`:

| Test File | Tests | Coverage |
| :--- | :---: | :--- |
| `wheresthematch_tests.rs` | 6 | HTML parsing, team matching, channels, error edge cases |
| `worldsoccertalk_tests.rs` | 5 | HTML parsing, channels, competition extraction, edge cases |
| `matchstv_tests.rs` | 15 | HTML parsing, French date parsing, time conversion, edge cases |
| `livesoccertv_tests.rs` | 7 | Top matches parsing, structure validation, known teams, error cases |

### Test Resources (`tests/resources/`)
- `wheresthematch.html` — Real HTML from WheresTheMatch.com
- `worldsoccertalk.html` — Real HTML from WorldSoccerTalk.com
- `matchstv.html` — Real HTML from Matchs.tv
- `livesoccertv.html` — Real HTML from LiveSoccerTV.com

## Resources & Libraries

| Library | Purpose |
| :--- | :--- |
| **Ratatui** | The core TUI framework for rendering the interface. |
| **Tokio** | Asynchronous runtime for non-blocking network requests. |
| **Crossterm** | Handles low-level terminal input/output events. |
| **Wreq** | High-performance HTTP client with TLS impurity/emulation support (replaces Reqwest). |
| **Scraper** | HTML parsing library using CSS selectors. |
| **Chrono** | Date and time manipulation. |
| **Chrono-TZ** | Timezone database for converting ET/Paris times to local. |
| **Serde** | Serialization for configuration files. |
| **Async-Trait** | Enables async methods in the `FootballProvider` trait. |
| **Mockall** | (dev) Trait mocking for unit tests. |

## External Data Sources
- **UK**: [WherestheMatch.com](https://www.wheresthematch.com)
- **US**: [WorldSoccerTalk.com](https://worldsoccertalk.com)
- **FR**: [Matchs.tv](https://matchs.tv)
