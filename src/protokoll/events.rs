use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(rename = "summary")]
    pub title: String,
    pub location: String,
    pub start: DateTime<Local>,
}

pub fn fetch_calendar_events(api_url: &Url, client: &Client) -> Result<Vec<Event>> {
    let endpoint = api_url.join("api/calendar/")?;

    let response = client
        .get(endpoint)
        .send()
        .context("unable to fetch current tops")?;

    let events = response.json().context("failed to deserialize events")?;

    Ok(events)
}
