use anyhow::{Context, Result};
use chrono::{DateTime, FixedOffset};
use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Deserialize, PartialEq, Eq, strum::Display, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
pub struct Sitzung {
    pub id: Uuid,
    pub datetime: DateTime<FixedOffset>,
    pub kind: SitzungKind,
    pub tops: Vec<Top>,
    pub antragsfrist: DateTime<FixedOffset>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Antrag {
    pub titel: String,
    pub antragstext: String,
    pub begründung: String,
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TopKind {
    Regularia,
    Bericht,
    Normal,
    Verschiedenes,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Top {
    pub weight: i64,
    pub name: String,
    pub anträge: Vec<Antrag>,
    pub kind: TopKind,
    pub inhalt: String,
}

pub fn fetch_sitzung(
    api_url: &Url,
    client: &Client,
    datetime: DateTime<FixedOffset>,
) -> Result<Sitzung> {
    let mut endpoint = api_url.join("api/sitzungen/after/")?;
    endpoint.set_query(Some(
        format!(
            "timestamp={}&limit=1",
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

    let sitzungen = response
        .json::<Vec<Sitzung>>()
        .context("failed to deserialize sitzung")?;
    let sitzung = sitzungen[0].clone();

    Ok(sitzung)
}
