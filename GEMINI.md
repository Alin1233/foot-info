# Project Architecture: Football Match Scraper

This document outlines the architecture and resources used in the **foot-info** TUI application.

## Overview
**foot-info** is a Terminal User Interface (TUI) application written in Rust. It allows users to search for a football team and scrapes match schedules and TV broadcast information from [WherestheMatch.com](https://www.wheresthematch.com).

## Architecture
The application follows a clean separation of concerns, modularized into specific layers:

### 1. **Entry Point (`src/main.rs`)**
- Initializes the Tokio async runtime (`#[tokio::main]`).
- Sets up the `Ratatui` terminal backend.
- Launches the main application loop (`App::run`).

### 2. **Application State (`src/app.rs`)**
- **Responsibility**: Manages the runtime state of the application.
- **Key Components**:
  - `search_input`: Stores the user's current query.
  - `matches`: A vector of `Match` structs containing the scraped data.
  - `is_loading` / `error_message`: UI state flags.
  - **Async Event Loop**: Uses `tokio::sync::mpsc` channels to handle asynchronous scraping tasks without freezing the UI. User input triggers a task spawn, and results are sent back to the main loop via the channel.

### 3. **UI Layer (`src/ui.rs` & `src/theme.rs`)**
- **Responsibility**: Renders the application to the terminal frame.
- **Key Components**:
  - **Layout**: A vertical stack separating the Title, Input Field, and Results/Status area.
  - **Theme**: A custom color palette ("BlackOrangeBeigeGoldFall") defined in `src/theme.rs` to ensure consistent styling.
  - **Widgets**: Uses `Paragraph` for text and `List` (customized) for displaying match cards.

### 4. **Data Layer (`src/models.rs`)**
- Defines the core data structures.
- `Match`: Holds `teams`, `competition`, `date`, `time`, and `channels`.

### 5. **Client / Scraping Engine (`src/client.rs`)**
- **Responsibility**: Fetches and parses data from the web.
- **Logic**:
  - **Fetch**: Uses `reqwest` (async) to GET the team page.
  - **Validation**: Checks for 404s, redirects, and "Invalid URL Format" error pages to detect incorrect team names.
  - **Parsing**: Uses `scraper` with CSS selectors to extract match details from the HTML table.
  - **Filtering**: Removes advertisements and header rows.
  - **Normalization**: Cleans up team names and channel strings.

### 6. **Utilities (`src/user.rs` & `src/error.rs`)**
- **`user.rs`**: Handles user-specific localization, specifically converting scraped UTC ISO timestamps to the local system time using `chrono` and `chrono-tz` (via `Local`).
- **`error.rs`**: Defines a custom `AppError` enum using `thiserror` to categorize issues (Network, TeamNotFound, NoMatchesScheduled) and provide user-friendly error messages.

## Resources & Libraries

| Library | Purpose |
| :--- | :--- |
| **Ratatui** | The core TUI framework for rendering the interface. |
| **Tokio** | Asynchronous runtime for non-blocking network requests and event handling. |
| **Crossterm** | Handles low-level terminal input/output events. |
| **Reqwest** | HTTP client for fetching HTML pages (JSON feature enabled, though not used for this scraping). |
| **Scraper** | HTML parsing library (similar to BeautifulSoup) using CSS selectors. |
| **Chrono** | Date and time manipulation. |
| **Thiserror** | Ergonomic error handling for defining custom error types. |

## External Data Source
- **Website**: [WherestheMatch.com](https://www.wheresthematch.com)
- **Method**: HTML Scraping.
- **Caveat**: The application relies on the specific DOM structure of the website. Changes to the website's layout will likely require updates to the CSS selectors in `src/client.rs`.
