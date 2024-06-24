use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Sitzung {
    #[serde(rename = "datum")]
    pub date: NaiveDateTime,
}

pub fn fetch_next_sitzung(api_url: &Url, client: &Client) -> Result<Sitzung> {
    let endpoint = api_url.join("api/topmanager/next_sitzung/")?;

    let response = client
        .get(endpoint)
        .send()
        .context("unable to fetch next sitzung")?;

    println!("{:?}", response);

    let sitzung = response.json().context("failed to deserialize sitzung")?;

    Ok(sitzung)
}
