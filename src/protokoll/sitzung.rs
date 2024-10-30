use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Deserialize, PartialEq, Eq, strum::Display)]
#[serde(rename_all = "lowercase")]
pub enum SitzungKind {
    #[strum(to_string = "normal")]
    Normal,
    #[strum(to_string = "vv")]
    VV,
    #[strum(to_string = "wahlvv")]
    WahlVV,
    #[strum(to_string = "ersatz")]
    Ersatz,
    #[strum(to_string = "konsti")]
    Konsti,
    #[strum(to_string = "dringlichkeit")]
    Dringlichkeit,
}

#[derive(Debug, Deserialize)]
pub struct Sitzung {
    pub id: Uuid,
    pub datetime: DateTime<Local>,
    pub kind: SitzungKind,
    pub tops: Vec<Top>,
}

#[derive(Debug, Deserialize)]
pub struct Antrag {
    pub titel: String,
    pub antragstext: String,
    pub begründung: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TopKind {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "sonstiges")]
    Verschiedenes,
}

#[derive(Debug, Deserialize)]
pub struct Top {
    pub weight: i64,
    pub name: String,
    pub anträge: Vec<Antrag>,
    pub kind: TopKind,
}

pub fn fetch_sitzung(api_url: &Url, client: &Client, datetime: DateTime<Local>) -> Result<Sitzung> {
    let mut endpoint = api_url.join("api/sitzungen/first-after/")?;
    endpoint.set_query(Some(
        format!(
            "timestamp={}",
            datetime
                .to_rfc3339()
                .replace(":", "%3A")
                .replace("+", "%2B")
        )
        .as_str(),
    ));

    let response = client
        .get(endpoint)
        .send()
        .context("unable to fetch next sitzung")?
        .error_for_status()
        .context("unable to fetch next sitzung")?;

    let sitzung = response.json().context("failed to deserialize sitzung")?;

    Ok(sitzung)
}
