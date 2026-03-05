# Project Architecture: Football Match Scraper

This document outlines the architecture and resources used in the **foot-info** TUI application.

## Overview
**foot-info** is a Terminal User Interface (TUI) application written in Rust. It allows users to search for a football team and scrapes match schedules and TV broadcast information from various sources based on the selected country.

## Architecture
The application follows a **Provider-based Architecture** with clear separation of concerns across state, event handling, UI, and data layers.

### 1. **Workspace Entry (`tui/src/main.rs`)**
- Initializes the Tokio async runtime (`#[tokio::main]`).
- Sets up the `Ratatui` terminal backend.
- Imports `App` from the `foot_info_tui` library crate and launches `App::run`.

### 2. **Core Library (`core/`)**
The `foot_info_core` crate contains all pure domain logic, independent of any UI framework.
- **API (`src/client.rs`)**: Exposes `FootballClient`, an orchestration layer that simplifies data fetching from various providers (`fetch_top_matches`, `search_team`). This layer is designed to be easily callable via FFI (e.g., from Flutter).
- **Domain Models (`src/models.rs`)**: Core data structures (`Match`, `TopMatch`, `Country`).
- **Time Utils (`src/utils/time.rs`)**: Timezone conversions.

### 3. **Terminal App (`tui/`)**
The `foot_info_tui` crate contains all interactive and visual terminal components, depending heavily on `foot_info_core`.

#### **State (`tui/src/state.rs`)**
- **Responsibility**: Terminal application data.
- **`AppState` struct** holds:
  - `client: FootballClient`
  - `search_input`, `matches`, `error_message`, `status_message`
  - `is_loading`, `exit`, `current_provider_index`
  - `view_mode`, `top_matches`, `selected_top_match_index`
  - `config: Config`
- Helper: `get_current_provider()` returns the currently active data provider.

#### **Event Handling (`tui/src/handlers/`)**
- **Responsibility**: Keybindings and mode transitions.
- **`mod.rs`**: Dispatcher logic, plus `handle_action` for processing async callback boundaries.
- **`search.rs`**: Search-mode keybindings.
- **`top_matches.rs`**: TopMatches-mode keybindings (chronological ↑/↓, column-hopping ←/→).

#### **Orchestrator (`tui/src/app.rs`)**
- **Responsibility**: Thin runtime shell — owns `AppState` + `mpsc` channels + the async run loop.
- `App::run()` coordinates: polls terminal events → delegates to `handlers` → calls `core`'s `FootballClient` → processes results.

### 4. **The Provider System (`core/src/providers/`)**
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

### 5. **UI Layer (`tui/src/ui/`)**
Modular component-based structure implementing **Dynamic Responsive Design**.

```
tui/src/ui/
├── mod.rs              # Re-exports draw()
├── render.rs           # Delegates outer frame rendering to views
├── layout.rs           # Reusable layout geometry (utilizes `Constraint::Fill()` / `Constraint::Max()`)
├── theme.rs            # Color constants (BG_BLACK, GOLD, RUST_ORANGE, BEIGE)
├── views/
│   ├── mod.rs
│   ├── search.rs            # Search view composition
│   └── top_matches.rs       # Top matches view composition
└── components/
    ├── mod.rs
    ├── search_bar.rs        # Search input widget
    ├── match_list.rs        # Results display
    ├── status_bar.rs        # Transient status messages
    └── top_matches_list.rs  # Upcoming top matches grid
```

### 6. **Data Models**
- **Core (`core/src/models.rs`)**: `Match`, `TopMatch`, `Country`.
- **TUI (`tui/src/models.rs`)**: `ViewMode` (Search, TopMatches).

### 7. **Error Handling (`core/src/error.rs`)**
- `AppError` enum with variants: `NetworkError`, `TeamNotFound`, `NoMatchesScheduled`.

### 8. **Utilities (`tui/src/config.rs`)**
- Manages persistence of user preferences (favorite team) using `serde` and the system's config directory.

### 9. **Known Issues & Build Fixes**
- **Android NDK Compilation (`cargokit` and `boringssl`)**: When building the Flutter application for Android, `boring-sys2` (a dependency of `wreq`) requires the `ANDROID_NDK_HOME` environment variable to compile C extensions. To fix `Please set ANDROID_NDK_HOME for Android build` errors, `ANDROID_NDK_HOME` was manually exported in the `buildEnvironment()` map within `app/rust_builder/cargokit/build_tool/lib/src/android_environment.dart`.

## Testing

### Integration Tests
Integration tests are isolated to avoid UI dependency where possible:
- **Core Logic (`core/tests/`)**: Tests HTML parsing and data extraction using real offline HTML stored in `core/tests/resources/`.
- **UI & State (`tui/tests/`)**: Tests state transition logic (`state_tests.rs`, `handler_tests.rs`) and Ratatui spatial rendering geometries (`ui/`).

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
