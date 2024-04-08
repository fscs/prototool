use anyhow::{Context, Result};
use askama::Template;
use chrono::NaiveDateTime;
use serde::Deserialize;
use url::Url;

use std::{fs, path::Path};

use crate::raete::Rat;

#[derive(Debug, Deserialize)]
pub struct Antrag {
    pub titel: String,
    pub antragstext: String,
    pub begründung: String,
}

#[derive(Debug, Deserialize)]
pub struct Top {
    pub weight: i64,
    pub name: String,
    pub anträge: Vec<Antrag>,
}

pub fn fetch_current_tops(api_url: &Url) -> Result<Vec<Top>> {
    let endpoint = api_url.join("api/topmanager/current_tops/")?;

    let response = reqwest::blocking::get(endpoint).context("unable to fetch current tops")?;

    let mut tops: Vec<Top> = response.json().context("failed to deserialize tops")?;

    #[allow(clippy::unwrap_used)]
    tops.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

    Ok(tops)
}

pub fn write_protokoll_template(
    path: &Path,
    tops: Vec<Top>,
    räte: Vec<Rat>,
    datetime: &NaiveDateTime,
) -> Result<()> {
    let date_machine = datetime.format("%Y-%m-%dT%H:%M:%S");
    let date_human = datetime.format("%d.%m.%Y");

    let template = ProtokollTemplate {
        date_machine: date_machine.to_string(),
        date: date_human.to_string(),
        tops,
        räte,
    };

    let result = template
        .render()
        .context("failed to render protokoll template")?;

    fs::write(path, result).context("failed to write protokoll template")?;

    Ok(())
}

#[derive(Debug, Template)]
#[template(path = "../templates/protokoll.md")]
struct ProtokollTemplate {
    pub tops: Vec<Top>,
    pub räte: Vec<Rat>,
    pub date: String,
    pub date_machine: String,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::raete::Rat;

    use super::{Antrag, ProtokollTemplate, Top};
    use askama::Template;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use pretty_assertions::assert_eq;

    use std::fs;

    static PROTOKOLL_NO_TOPS: &'static str = include_str!("../tests/protokoll-no-tops.md");
    static PROTOKOLL_WITH_TOPS: &'static str = include_str!("../tests/protokoll-with-tops.md");
    static PROTOKOLL_WITH_RÄTE: &'static str = include_str!("../tests/protokoll-with-rate.md");

    #[test]
    fn render_without_tops() {
        let template = ProtokollTemplate {
            date: "27.05.2022".to_string(),
            date_machine: "2022-05-27T07:30:15".to_string(),
            tops: vec![],
            räte: vec![],
        };

        assert_eq!(template.render().unwrap(), PROTOKOLL_NO_TOPS);
    }

    #[test]
    fn render_with_tops() {
        let template = ProtokollTemplate {
            date: "27.05.2022".to_string(),
            date_machine: "2022-05-27T07:30:15".to_string(),
            räte: vec![],
            tops: vec![
                Top {
                    name: "Blumen für Valentin".to_string(),
                    weight: 1,
                    anträge: vec![Antrag {
                        titel: "Blumen für Valentin".to_string(),
                        antragstext: "Die Fachschaft Informatik beschließt".to_string(),
                        begründung: "Weil wir Valentin toll finden".to_string(),
                    }],
                },
                Top {
                    name: "Volt Zapfanlage".to_string(),
                    weight: 2,
                    anträge: vec![
                        Antrag {
                            titel: "Tank für Voltzapfanlage".to_string(),
                            antragstext: "Die Fachschaft Informatik beschließt".to_string(),
                            begründung: "Volt aus dem Hahn > Volt aus der Dose".to_string(),
                        },
                        Antrag {
                            titel: "Hahn für Voltzapfanlage".to_string(),
                            antragstext: "Die Fachschaft Informatik beschließt".to_string(),
                            begründung: "Volt aus dem Hahn > Volt aus der Dose".to_string(),
                        },
                    ],
                },
            ],
        };

        assert_eq!(template.render().unwrap(), PROTOKOLL_WITH_TOPS);
    }

    #[test]
    fn render_with_räte() {
        let template = ProtokollTemplate {
            date: "27.05.2022".to_string(),
            date_machine: "2022-05-27T07:30:15".to_string(),
            räte: vec![
                Rat {
                    name: "Valentin".to_string(),
                    abgemeldet: false
                },
                Rat {
                    name: "Jonas \"Kooptimus\"".to_string(),
                    abgemeldet: false
                },
                Rat {
                    name: "Marcel \"Markal\"".to_string(),
                    abgemeldet: false
                },
                Rat {
                    name: "Elif".to_string(),
                    abgemeldet: true
                },
                Rat {
                    name: "Australian".to_string(),
                    abgemeldet: true
                },
            ],
            tops: vec![],
        };

        assert_eq!(template.render().unwrap(), PROTOKOLL_WITH_RÄTE);
    }

    #[test]
    fn write_protokoll_template() {
        let tmpfile = tempfile::NamedTempFile::new().unwrap();
        let time = NaiveTime::from_hms_opt(7, 30, 15).unwrap();
        let date = NaiveDate::from_ymd_opt(2022, 5, 27).unwrap();
        let datetime = NaiveDateTime::new(date, time);

        let tops = vec![
            Top {
                name: "Blumen für Valentin".to_string(),
                weight: 1,
                anträge: vec![Antrag {
                    titel: "Blumen für Valentin".to_string(),
                    antragstext: "Die Fachschaft Informatik beschließt".to_string(),
                    begründung: "Weil wir Valentin toll finden".to_string(),
                }],
            },
            Top {
                name: "Volt Zapfanlage".to_string(),
                weight: 2,
                anträge: vec![
                    Antrag {
                        titel: "Tank für Voltzapfanlage".to_string(),
                        antragstext: "Die Fachschaft Informatik beschließt".to_string(),
                        begründung: "Volt aus dem Hahn > Volt aus der Dose".to_string(),
                    },
                    Antrag {
                        titel: "Hahn für Voltzapfanlage".to_string(),
                        antragstext: "Die Fachschaft Informatik beschließt".to_string(),
                        begründung: "Volt aus dem Hahn > Volt aus der Dose".to_string(),
                    },
                ],
            },
        ];

        super::write_protokoll_template(tmpfile.path(), tops, vec![], &datetime).unwrap();

        assert_eq!(fs::read_to_string(tmpfile).unwrap(), PROTOKOLL_WITH_TOPS);
    }
}
