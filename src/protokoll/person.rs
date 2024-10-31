use std::fmt::{Display, Formatter};

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

use super::Sitzung;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PersonWithAbmeldung {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub abgemeldet: bool,
    pub anwesend: bool,
}

impl Display for PersonWithAbmeldung {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {}",
            self.first_name.as_str(),
            self.last_name.as_str()
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Abmeldung {
    pub person_id: Uuid,
}

pub fn fetch_raete(api_url: &Url, client: &Client) -> Result<Vec<Person>> {
    let mut endpoint = api_url.join("api/persons/by-role/")?;
    endpoint.set_query(Some("role=Rat"));

    let response = client
        .get(endpoint)
        .send()
        .context("unable to fetch räte")?
        .error_for_status()
        .context("unable to fetch räte")?;

    let persons = response.json().context("unable to deserialize räte")?;

    Ok(persons)
}

pub fn fetch_abmeldungen(
    api_url: &Url,
    client: &Client,
    sitzung: &Sitzung,
) -> Result<Vec<Abmeldung>> {
    let endpoint = api_url.join(format!("api/sitzungen/{}/abmeldungen/", sitzung.id).as_str())?;

    let response = client
        .get(endpoint)
        .send()
        .context("unable to fetch abmeldungen")?
        .error_for_status()
        .context("unable to fetch abmeldungen")?;

    let abmeldungen = response
        .json()
        .context("unable to deserialize abmeldungen")?;

    Ok(abmeldungen)
}

pub fn determine_abgemeldet_räte(
    personen: &[Person],
    abmeldungen: &[Abmeldung],
) -> Vec<PersonWithAbmeldung> {
    personen
        .iter()
        .map(|p| {
            let abgemeldet = abmeldungen.iter().any(|a| a.person_id == p.id);

            PersonWithAbmeldung {
                id: p.id,
                first_name: p.first_name.to_owned(),
                last_name: p.last_name.to_owned(),
                anwesend: false,
                abgemeldet,
            }
        })
        .collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{Abmeldung, Person, PersonWithAbmeldung};
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    #[test]
    fn determine_abmeldungen() {
        let persons = vec![
            Person {
                id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
                first_name: "Valentin".to_string(),
                last_name: "Pukhov".to_string(),
            },
            Person {
                id: Uuid::parse_str("444e8400-e29b-41d4-a716-446655440000").unwrap(),
                first_name: "Florian".to_string(),
                last_name: "Schubert".to_string(),
            },
        ];

        let abmeldungen = vec![Abmeldung {
            person_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
        }];

        let expected = vec![
            PersonWithAbmeldung {
                id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
                first_name: "Valentin".to_string(),
                last_name: "Pukhov".to_string(),
                abgemeldet: true,
                anwesend: false,
            },
            PersonWithAbmeldung {
                id: Uuid::parse_str("444e8400-e29b-41d4-a716-446655440000").unwrap(),
                first_name: "Florian".to_string(),
                last_name: "Schubert".to_string(),
                abgemeldet: false,
                anwesend: false,
            },
        ];

        let actual = super::determine_abgemeldet_räte(&persons, &abmeldungen);

        assert_eq!(expected, actual);
    }
}
