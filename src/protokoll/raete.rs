use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
pub struct Rat {
    pub name: String,
    pub abgemeldet: bool,
}

#[derive(Debug, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Abmeldung {
    pub person_id: Uuid,
}

pub fn fetch_persons(
    api_url: &Url,
    client: &Client,
    datetime: &NaiveDate,
) -> Result<Vec<Person>> {
    let endpoint = api_url.join("api/person/by-role/")?;

    let datestr = datetime.format("%Y-%m-%d").to_string();
    let mut params = HashMap::new();
    params.insert("rolle", "Rat");
    params.insert("anfangsdatum", datestr.as_str());
    params.insert("ablaufdatum", datestr.as_str());

    let response = client
        .get(endpoint)
        .json(&params)
        .send()
        .context("unable to fetch räte")?;
    
    let persons = response.json().context("unable to deserialize räte")?;

    Ok(persons)
}

pub fn fetch_abmeldungen(api_url: &Url, client: &Client, datetime: &NaiveDate) -> Result<Vec<Abmeldung>> {
    let endpoint = api_url.join("api/abmeldungen/between/")?;

    let datestr = datetime.format("%Y-%m-%d").to_string();
    let mut params = HashMap::new();
    params.insert("start", datestr.as_str());
    params.insert("end", datestr.as_str());

    let response = client
        .get(endpoint)
        .json(&params)
        .send()
        .context("unable to fetch abmeldungen")?;

    let abmeldungen = response
        .json()
        .context("unable to deserialize abmeldungen")?;

    Ok(abmeldungen)
}

pub fn determine_present_räte(personen: &[Person], abmeldungen: &[Abmeldung]) -> Vec<Rat> {
    personen
        .iter()
        .map(|p| {
            let abgemeldet = abmeldungen.iter().any(|a| a.person_id == p.id);

            Rat {
                name: p.name.to_owned(),
                abgemeldet,
            }
        })
        .collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{Abmeldung, Person, Rat};
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    #[test]
    fn determine_abmeldungen() {
        let persons = vec![
            Person {
                id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
                name: "Valentin Pukhov".to_string(),
            },
            Person {
                id: Uuid::parse_str("444e8400-e29b-41d4-a716-446655440000").unwrap(),
                name: "Florian Schubert".to_string(),
            },
        ];

        let abmeldungen = vec![Abmeldung {
            person_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
        }];

        let expected = vec![
            Rat {
                name: "Valentin Pukhov".to_string(),
                abgemeldet: true,
            },
            Rat {
                name: "Florian Schubert".to_string(),
                abgemeldet: false,
            },
        ];

        let actual = super::determine_present_räte(&persons, &abmeldungen);

        assert_eq!(expected, actual);
    }
}
