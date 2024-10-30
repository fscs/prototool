use anyhow::{anyhow, bail, Result};
use askama::Template;
use chrono::NaiveDate;
use markdown::mdast;
use serde::Deserialize;

pub mod events;
pub mod person;
pub mod sitzung;

pub use events::Event;
pub use person::{Abmeldung, Person, PersonWithAbmeldung};
pub use sitzung::{Antrag, Sitzung, SitzungKind, Top, TopKind};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProtokollFrontmatter {
    pub date: Option<NaiveDate>,
    pub lastmod: Option<NaiveDate>,
    pub sitzung_kind: Option<SitzungKind>,
}

pub fn find_frontmatter(protokoll: &mdast::Node) -> Result<ProtokollFrontmatter> {
    let Some(children) = protokoll.children() else {
        bail!("document is empty");
    };

    let frontmatter = children
        .iter()
        .find(|e| matches!(e, mdast::Node::Toml(_) | mdast::Node::Yaml(_)))
        .ok_or_else(|| anyhow!("no frontmatter found"))?;

    let result: ProtokollFrontmatter = match frontmatter {
        mdast::Node::Toml(toml) => toml::from_str(toml.value.as_str())?,
        mdast::Node::Yaml(yaml) => serde_yaml::from_str(yaml.value.as_str())?,
        _ => unreachable!(),
    };

    Ok(result)
}

/// finds the creation date of this protokoll by searching the frontmatter for
/// the keys 'date' or 'lastmod'
pub fn find_protokoll_date(frontmatter: &ProtokollFrontmatter) -> Result<NaiveDate> {
    if let Some(date) = frontmatter.date {
        return Ok(date);
    } else if let Some(lastmod) = frontmatter.lastmod {
        return Ok(lastmod);
    } else {
        bail!("neither 'date' or 'lastmod' set in frontmatter")
    }
}

#[derive(Debug, Template)]
#[template(path = "../templates/protokoll.md")]
pub struct ProtokollTemplate {
    pub sitzung: Sitzung,
    pub raete: Vec<PersonWithAbmeldung>,
    pub events: Vec<Event>,
}

// these are functions available within the template
mod filters {
    use chrono::{DateTime, Days, Local, NaiveDate};

    use super::{Event, PersonWithAbmeldung, Sitzung, SitzungKind, Top, TopKind};

    pub fn normal_tops(tops: &[Top]) -> askama::Result<Vec<&Top>> {
        let result = tops.iter().filter(|e| e.kind == TopKind::Normal).collect();

        Ok(result)
    }

    pub fn verschiedenes_tops(tops: &[Top]) -> askama::Result<Vec<&Top>> {
        let result = tops
            .iter()
            .filter(|e| e.kind == TopKind::Verschiedenes)
            .collect();

        Ok(result)
    }

    pub fn hidden_until_date(datetime: &DateTime<Local>) -> askama::Result<NaiveDate> {
        let date = datetime.date_naive();
        let result = date.checked_add_days(Days::new(4)).unwrap_or(date);

        Ok(result)
    }

    pub fn event_format(event: &Event) -> askama::Result<String> {
        let result = format!(
            "{} {} {} Uhr {}",
            event.start.format("%d.%m."),
            event.title.as_ref().map_or("", |e| e.as_str()),
            event.start.format("%H:%M"),
            event.location.as_ref().map_or("", |e| e.as_str()),
        );

        Ok(result)
    }

    pub fn protokoll_title(sitzung: &Sitzung) -> askama::Result<String> {
        let prefix = match &sitzung.kind {
            SitzungKind::VV | SitzungKind::WahlVV => "VV-Protokoll",
            SitzungKind::Konsti => "Konsti-Protokoll",
            _ => "Protokoll",
        };

        let result = format!("{} vom {}", prefix, sitzung.datetime.format("%d.%m.%Y"));

        Ok(result)
    }

    pub fn anwesende_raete_label(raete: &[PersonWithAbmeldung]) -> askama::Result<String> {
        let anwesend_count = raete.iter().filter(|r| r.anwesend).count();

        if anwesend_count == 0 {
            Ok("n".to_string()) // i mean at least one has got to be the person writing the
                                // protocoll
        } else {
            Ok(anwesend_count.to_string())
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{
        person::PersonWithAbmeldung,
        sitzung::{Antrag, Sitzung, SitzungKind, Top, TopKind},
    };

    use super::ProtokollTemplate;
    use askama::Template;
    use chrono::{FixedOffset, NaiveDate};
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    static PROTOKOLL_NO_TOPS: &str = include_str!("../../tests/protokoll-no-tops.md");
    static PROTOKOLL_VV: &str = include_str!("../../tests/protokoll-vv.md");
    static PROTOKOLL_WITH_TOPS: &str = include_str!("../../tests/protokoll-with-tops.md");
    static PROTOKOLL_WITH_RÄTE: &str = include_str!("../../tests/protokoll-with-rate.md");

    // im quite sure we still fuck up the timezone faking and im not sure if we can actually do
    // anything about it when using DateTime<Local>. ATM this isnt a problem tho because we dont
    // care about the Time anyway and only the Date matters

    fn tz_offset() -> FixedOffset {
        FixedOffset::east_opt(3 * 60 * 60).unwrap()
    }

    #[test]
    fn render_without_tops() {
        let template = ProtokollTemplate {
            sitzung: Sitzung {
                id: Uuid::parse_str("efc794db-5d32-4186-a7d6-5fe6eee70452").unwrap(),
                datetime: NaiveDate::from_ymd_opt(2022, 5, 27)
                    .unwrap()
                    .and_hms_opt(7, 30, 15)
                    .unwrap()
                    .and_local_timezone(tz_offset())
                    .unwrap()
                    .into(),
                kind: SitzungKind::Normal,
                tops: vec![],
            },
            raete: vec![],
            events: vec![],
        };

        assert_eq!(template.render().unwrap(), PROTOKOLL_NO_TOPS);
    }

    #[test]
    fn render_vv() {
        let template = ProtokollTemplate {
            sitzung: Sitzung {
                id: Uuid::parse_str("efc794db-5d32-4186-a7d6-5fe6eee70452").unwrap(),
                datetime: NaiveDate::from_ymd_opt(2022, 5, 27)
                    .unwrap()
                    .and_hms_opt(7, 30, 15)
                    .unwrap()
                    .and_local_timezone(tz_offset())
                    .unwrap()
                    .into(),
                kind: SitzungKind::VV,
                tops: vec![],
            },
            raete: vec![],
            events: vec![],
        };

        assert_eq!(template.render().unwrap(), PROTOKOLL_VV);
    }

    #[test]
    fn render_with_tops() {
        let template = ProtokollTemplate {
            sitzung: Sitzung {
                id: Uuid::parse_str("efc794db-5d32-4186-a7d6-5fe6eee70452").unwrap(),
                datetime: NaiveDate::from_ymd_opt(2022, 5, 27)
                    .unwrap()
                    .and_hms_opt(7, 30, 15)
                    .unwrap()
                    .and_local_timezone(tz_offset())
                    .unwrap()
                    .into(),
                kind: SitzungKind::Normal,
                tops: vec![
                    Top {
                        name: "Blumen für Valentin".to_string(),
                        weight: 1,
                        kind: TopKind::Normal,
                        anträge: vec![Antrag {
                            titel: "Blumen für Valentin".to_string(),
                            antragstext: "Die Fachschaft Informatik beschließt".to_string(),
                            begründung: "Weil wir Valentin toll finden".to_string(),
                        }],
                    },
                    Top {
                        name: "Voltpfand".to_string(),
                        weight: 1,
                        kind: TopKind::Verschiedenes,
                        anträge: vec![Antrag {
                            titel: "Voltpfand".to_string(),
                            antragstext: "aint nobody got time for that".to_string(),
                            begründung: "Der Voltpfand muss dringend weggebracht werden."
                                .to_string(),
                        }],
                    },
                    Top {
                        name: "Volt Zapfanlage".to_string(),
                        weight: 2,
                        kind: TopKind::Normal,
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
            },
            events: vec![],
            raete: vec![],
        };

        assert_eq!(template.render().unwrap(), PROTOKOLL_WITH_TOPS);
    }

    #[test]
    fn render_with_räte() {
        let template = ProtokollTemplate {
            sitzung: Sitzung {
                id: Uuid::parse_str("efc794db-5d32-4186-a7d6-5fe6eee70452").unwrap(),
                datetime: NaiveDate::from_ymd_opt(2022, 5, 27)
                    .unwrap()
                    .and_hms_opt(7, 30, 15)
                    .unwrap()
                    .and_local_timezone(tz_offset())
                    .unwrap()
                    .into(),
                kind: SitzungKind::Normal,
                tops: vec![],
            },
            raete: vec![
                PersonWithAbmeldung {
                    name: "Valentin".to_string(),
                    abgemeldet: false,
                    anwesend: true,
                },
                PersonWithAbmeldung {
                    name: "Jonas \"Kooptimus\"".to_string(),
                    abgemeldet: false,
                    anwesend: false,
                },
                PersonWithAbmeldung {
                    name: "Marcel \"Markal\"".to_string(),
                    abgemeldet: false,
                    anwesend: false,
                },
                PersonWithAbmeldung {
                    name: "Elif".to_string(),
                    abgemeldet: true,
                    anwesend: false,
                },
                PersonWithAbmeldung {
                    name: "Australian".to_string(),
                    abgemeldet: true,
                    anwesend: false,
                },
            ],
            events: vec![],
        };

        assert_eq!(template.render().unwrap(), PROTOKOLL_WITH_RÄTE);
    }

    #[test]
    fn find_protokoll_date() {
        let protokoll = r#"---
title: "Protokoll vom 27.05.2022"
date: "2022-05-27"
---
        "#;

        let markdown_opts = markdown::ParseOptions {
            constructs: markdown::Constructs {
                frontmatter: true,
                ..Default::default()
            },
            ..markdown::ParseOptions::default()
        };

        let mdast = markdown::to_mdast(protokoll, &markdown_opts).unwrap();
        let frontmatter = super::find_frontmatter(&mdast).unwrap();

        let timestamp = super::find_protokoll_date(&frontmatter).unwrap();

        let expected = NaiveDate::from_ymd_opt(2022, 5, 27).unwrap();

        assert_eq!(timestamp, expected);
    }

    #[test]
    fn find_protokoll_lastmod() {
        let protokoll = r#"---
title: "Protokoll vom 27.05.2022"
lastmod: "2022-05-27"
---
        "#;

        let markdown_opts = markdown::ParseOptions {
            constructs: markdown::Constructs {
                frontmatter: true,
                ..markdown::Constructs::gfm()
            },
            ..markdown::ParseOptions::default()
        };

        let mdast = markdown::to_mdast(protokoll, &markdown_opts).unwrap();
        let frontmatter = super::find_frontmatter(&mdast).unwrap();

        let timestamp = super::find_protokoll_date(&frontmatter).unwrap();

        let expected = NaiveDate::from_ymd_opt(2022, 5, 27).unwrap();

        assert_eq!(timestamp, expected);
    }
}
