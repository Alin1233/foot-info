use crate::error::AppError;
use crate::models::{LeagueFixture, LeagueStats, StandingRow, TopScorer};
use chrono::{Local, TimeZone, Utc};
use scraper::{Html, Selector};
use wreq::Client;
use wreq_util::Emulation;

/// Fetches and parses a LiveSoccerTV competition page.
///
/// Example URL: `https://www.livesoccertv.com/competitions/england/premier-league/`
pub async fn fetch_league_stats(competition_url: &str) -> Result<LeagueStats, AppError> {
    let client = Client::builder()
        .emulation(Emulation::Chrome136)
        .build()?;

    let response = client
        .get(competition_url)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(AppError::ProviderError(format!(
            "HTTP {} from livesoccertv.com",
            response.status()
        )));
    }

    let body: String = response
        .text()
        .await?;

    if body.is_empty() {
        return Err(AppError::NoMatchesScheduled(
            "Empty response from livesoccertv.com".to_string(),
        ));
    }

    parse_html(&body)
}

fn timestamp_to_local(millis: i64) -> (String, String) {
    match Utc.timestamp_millis_opt(millis).single() {
        Some(dt) => {
            let local = dt.with_timezone(&Local);
            (
                local.format("%a %d %b %Y").to_string(),
                local.format("%H:%M").to_string(),
            )
        }
        None => ("Unknown".to_string(), "??:??".to_string()),
    }
}

pub fn parse_html(body: &str) -> Result<LeagueStats, AppError> {
    let document = Html::parse_document(body);

    let competition = parse_competition_name(&document);
    let fixtures = parse_fixtures(&document);
    let table = parse_table(&document);
    let top_scorers = parse_top_scorers(&document);

    if fixtures.is_empty() && table.is_empty() {
        return Err(AppError::NoMatchesScheduled(
            "No league data found on this page".to_string(),
        ));
    }

    Ok(LeagueStats {
        competition,
        fixtures,
        table,
        top_scorers,
    })
}

fn parse_competition_name(document: &Html) -> String {
    let sel = Selector::parse("h1").expect("Invalid selector");
    document
        .select(&sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
        .unwrap_or_else(|| "Unknown Competition".to_string())
}

fn parse_fixtures(document: &Html) -> Vec<LeagueFixture> {
    let mut fixtures = Vec::new();

    // LiveSoccerTV uses a table with class "schedules" for fixtures.
    // Rows alternate between date headers (class "dheader") and match rows.
    let table_sel = Selector::parse("table.schedules").expect("Invalid selector");
    let tr_sel = Selector::parse("tr").expect("Invalid selector");
    let td_sel = Selector::parse("td").expect("Invalid selector");
    let span_ts_sel = Selector::parse("span.ts").expect("Invalid selector");
    let a_sel = Selector::parse("a").expect("Invalid selector");

    let table = match document.select(&table_sel).next() {
        Some(t) => t,
        None => return fixtures,
    };

    let mut current_date = String::new();

    for row in table.select(&tr_sel) {
        let classes: Vec<&str> = row.value().classes().collect();

        // Date header row
        if classes.contains(&"dheader") {
            current_date = row.text().collect::<String>().trim().to_string();
            continue;
        }

        let cells: Vec<_> = row.select(&td_sel).collect();
        if cells.len() < 3 {
            continue;
        }

        // Time cell — try `span.ts[dv]` for unix timestamp, fall back to text
        let time_cell = &cells[0];
        let (date, time) = time_cell
            .select(&span_ts_sel)
            .next()
            .and_then(|s| s.value().attr("dv"))
            .and_then(|dv| dv.parse::<i64>().ok())
            .map(timestamp_to_local)
            .unwrap_or_else(|| {
                let t = time_cell.text().collect::<String>().trim().to_string();
                (current_date.clone(), t)
            });

        let match_link = cells[1].select(&a_sel).next();
        let (home_team, away_team, score, match_url) = match match_link {
            Some(a) => {
                let mut h = String::new();
                let mut aw = String::new();
                let mut sc = String::new();
                let url = a.value().attr("href").unwrap_or("").to_string();
                
                let score_sel = Selector::parse("score").unwrap();
                if let Some(s_node) = a.select(&score_sel).next() {
                    sc = s_node.text().collect::<String>().trim().to_string();
                    let full_txt = a.text().collect::<String>();
                    let parts: Vec<&str> = full_txt.split(&sc).collect();
                    if parts.len() >= 2 {
                        h = parts[0].trim().to_string();
                        aw = parts[1].trim().to_string();
                    }
                } else {
                    let full = a.text().collect::<String>();
                    let full = full.trim();
                    if let Some(idx) = full.find(" vs ") {
                        h = full[..idx].trim().to_string();
                        aw = full[idx + 4..].trim().to_string();
                    } else if let Some(idx) = full.find(" - ") {
                        h = full[..idx].trim().to_string();
                        aw = full[idx + 3..].trim().to_string();
                    } else {
                        h = full.to_string();
                    }
                }
                (h, aw, if sc.is_empty() { None } else { Some(sc) }, url)
            }
            None => continue,
        };

        if home_team.is_empty() {
            continue;
        }

        // Channels — last cell, comma-separated text
        let channels = cells
            .last()
            .map(|c| {
                c.text()
                    .collect::<String>()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default();

        fixtures.push(LeagueFixture {
            home_team,
            away_team,
            date,
            time,
            score,
            channels,
            match_url,
        });
    }

    fixtures
}

fn parse_table(document: &Html) -> Vec<StandingRow> {
    let mut rows = Vec::new();

    let table_sel = Selector::parse("table.standings, table#fixtures").expect("Invalid selector");
    let tr_sel = Selector::parse("tbody tr").expect("Invalid selector");
    let td_sel = Selector::parse("td").expect("Invalid selector");

    let table = match document.select(&table_sel).next() {
        Some(t) => t,
        None => return rows,
    };

    for (i, row) in table.select(&tr_sel).enumerate() {
        let cells: Vec<_> = row.select(&td_sel).collect();
        if cells.len() < 9 {
            continue;
        }

        let text = |idx: usize| if idx < cells.len() { cells[idx].text().collect::<String>().trim().to_string() } else { String::new() };
        let parse_u8 = |idx: usize| text(idx).parse::<u8>().unwrap_or(0);
        let parse_u16 = |idx: usize| text(idx).parse::<u16>().unwrap_or(0);
        let parse_i16 = |idx: usize| text(idx).parse::<i16>().unwrap_or(0);

        // Form: last cell typically has spans like W/D/L
        let form_sel = Selector::parse("span").expect("Invalid selector");
        let form: Vec<char> = cells
            .last()
            .map(|c| {
                c.select(&form_sel)
                    .filter_map(|s| {
                        let t = s.text().collect::<String>();
                        let ch = t.trim().chars().next()?;
                        if matches!(ch, 'W' | 'D' | 'L') { Some(ch) } else { None }
                    })
                    .collect()
            })
            .unwrap_or_default();

        let offset = if cells.len() > 1 && cells[1].text().collect::<String>().trim().is_empty() { 1 } else { 0 };

        rows.push(StandingRow {
            position: text(0).parse::<u8>().unwrap_or((i as u8) + 1),
            team: text(1 + offset),
            played: parse_u8(2 + offset),
            won: parse_u8(3 + offset),
            drawn: parse_u8(4 + offset),
            lost: parse_u8(5 + offset),
            goals_for: parse_u16(6 + offset),
            goals_against: parse_u16(7 + offset),
            goal_diff: parse_i16(8 + offset),
            points: parse_u16(9 + offset),
            form,
        });
    }

    rows
}

fn parse_top_scorers(document: &Html) -> Vec<TopScorer> {
    let mut scorers = Vec::new();

    let table_sel = Selector::parse("table#topscorers-table, table#top_scorers, table.scorers").expect("Invalid selector");
    let tr_sel = Selector::parse("tbody tr").expect("Invalid selector");
    let td_sel = Selector::parse("td").expect("Invalid selector");

    let table = match document.select(&table_sel).next() {
        Some(t) => t,
        None => return scorers,
    };

    for row in table.select(&tr_sel) {
        let cells: Vec<_> = row.select(&td_sel).collect();
        if cells.len() < 3 {
            continue;
        }

        let text = |idx: usize| cells[idx].text().collect::<String>().trim().to_string();

        scorers.push(TopScorer {
            player: text(0),
            team: text(1),
            goals: text(2).parse().unwrap_or(0),
            penalties: if cells.len() > 3 { text(3).parse().unwrap_or(0) } else { 0 },
        });
    }

    scorers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_live() {
        let res = fetch_league_stats("https://www.livesoccertv.com/competitions/england/premier-league/").await;
        println!("RESULT: {:?}", res);
        assert!(res.is_ok());
    }
}
