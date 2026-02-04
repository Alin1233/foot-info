# ⚽ Foot Info - Football Match Scraper TUI

**Foot Info** is a fast, asynchronous Terminal User Interface application built in Rust. It allows you to check upcoming TV schedules and match fixtures for your favorite football teams directly from the command line, with support for multiple regions.

![Foot Info Demo](ss/Startup.png)

## 📸 Screenshots

| Startup Screen | Search Results |
|:---:|:---:|
| ![Startup](ss/Startup.png) | ![Results](ss/ResultsPagepng.png) |

## 🚀 Features

*   **Multi-Region Support**: Switch between **UK**, **US**, and **FR** data sources to see local TV listings.
*   **Real-time Scraping**: Fetches live data from high-quality sources:
    *   🇬🇧 **UK**: [WherestheMatch.com](https://www.wheresthematch.com)
    *   🇺🇸 **US**: [WorldSoccerTalk.com](https://worldsoccertalk.com)
    *   🇫🇷 **FR**: [Matchs.tv](https://matchs.tv)
*   **Favorite Team Persistence**: Save your favorite team to a configuration file for instant access.
*   **Local Time Conversion**: Automatically converts match kickoff times from UTC, ET (US), or Paris time to your local system time.
*   **Beautiful TUI**: Built with `ratatui` featuring a custom "Fall" color theme.

## 🛠️ Tech Stack

*   **Language**: Rust
*   **TUI Framework**: [Ratatui](https://github.com/ratatui-org/ratatui)
*   **Async Runtime**: [Tokio](https://tokio.rs/)
*   **HTTP Client**: [Reqwest](https://github.com/seanmonstar/reqwest)
*   **HTML Parsing**: [Scraper](https://github.com/causal-agent/scraper)
*   **Serialization**: [Serde](https://serde.rs/) (for config management)

## 📦 Installation

Ensure you have Rust and Cargo installed. If not, get them from [rustup.rs](https://rustup.rs/).

```bash
git clone https://github.com/your-username/foot-info.git
cd foot-info
cargo build --release
```

## 🎮 Usage

Run the application using Cargo:

```bash
cargo run
```

### Controls

*   **Type to Search**: Enter the name of a football team.
*   **`<Enter>`**: Submit the search.
*   **`<c>`**: Cycle through available regions (**UK**, **US**, **FR**).
*   **`<Ctrl+s>`**: Save current team as your favorite.
*   **`<Ctrl+f>`**: Load and search for your favorite team.
*   **`<Esc>`**: Quit the application.

## 📂 Project Structure

*   `src/main.rs`: Application entry point.
*   `src/app.rs`: State management and event loop.
*   `src/providers/`: Data source implementations (Strategy Pattern).
*   `src/config.rs`: User configuration and persistence logic.
*   `src/ui.rs`: Rendering logic and layout.
*   `src/user.rs`: Utilities for timezone conversion.
*   `src/theme.rs`: Color palette definitions.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This tool is for educational purposes. Data is scraped from third-party websites. Please respect their terms of service and usage policies.