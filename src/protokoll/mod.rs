use askama::Template;
use chrono::NaiveDateTime;

pub mod events;
pub mod raete;
pub mod tops;

pub use events::Event;
pub use raete::{Abmeldung, Person, Rat};
pub use tops::{Antrag, Top, TopType};

#[derive(Debug, Template)]
#[template(path = "../templates/protokoll.md")]
pub struct ProtokollTemplate {
    pub tops: Vec<Top>,
    pub raete: Vec<Rat>,
    pub events: Vec<Event>,
    pub datetime: NaiveDateTime,
}

mod filters {
    use chrono::{Days, NaiveDate, NaiveDateTime};

    use super::tops::{Top, TopType};

    pub fn normal_tops(tops: &[Top]) -> askama::Result<Vec<&Top>> {
        let result = tops
            .iter()
            .filter(|e| e.top_type == TopType::Normal)
            .collect();

        Ok(result)
    }

    pub fn sonstige_tops(tops: &[Top]) -> askama::Result<Vec<&Top>> {
        let result = tops
            .iter()
            .filter(|e| e.top_type == TopType::Sonstige)
            .collect();

        Ok(result)
    }

    pub fn hidden_until_date(datetime: &NaiveDateTime) -> askama::Result<NaiveDate> {
        let date = datetime.date();
        let result = date.checked_add_days(Days::new(4)).unwrap_or(date);

        Ok(result)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{
        events::Event,
        raete::Rat,
        tops::{Antrag, Top, TopType},
    };

    use super::ProtokollTemplate;
    use askama::Template;
    use chrono::{NaiveDate, TimeZone, Utc};
    use pretty_assertions::assert_eq;

    static PROTOKOLL_NO_TOPS: &str = include_str!("../../tests/protokoll-no-tops.md");
    static PROTOKOLL_WITH_TOPS: &str = include_str!("../../tests/protokoll-with-tops.md");
    static PROTOKOLL_WITH_RÄTE: &str = include_str!("../../tests/protokoll-with-rate.md");
    static PROTOKOLL_WITH_EVENTS: &str = include_str!("../../tests/protokoll-with-events.md");

    #[test]
    fn render_without_tops() {
        let template = ProtokollTemplate {
            datetime: NaiveDate::from_ymd_opt(2022, 5, 27)
                .unwrap()
                .and_hms_opt(7, 30, 15)
                .unwrap(),
            raete: vec![],
            events: vec![],
            tops: vec![],
        };

        assert_eq!(template.render().unwrap(), PROTOKOLL_NO_TOPS);
    }

    #[test]
    fn render_with_tops() {
        let template = ProtokollTemplate {
            datetime: NaiveDate::from_ymd_opt(2022, 5, 27)
                .unwrap()
                .and_hms_opt(7, 30, 15)
                .unwrap(),
            events: vec![],
            raete: vec![],
            tops: vec![
                Top {
                    name: "Blumen für Valentin".to_string(),
                    weight: 1,
                    top_type: TopType::Normal,
                    anträge: vec![Antrag {
                        titel: "Blumen für Valentin".to_string(),
                        antragstext: "Die Fachschaft Informatik beschließt".to_string(),
                        begründung: "Weil wir Valentin toll finden".to_string(),
                    }],
                },
                Top {
                    name: "Voltpfand".to_string(),
                    weight: 1,
                    top_type: TopType::Sonstige,
                    anträge: vec![Antrag {
                        titel: "Voltpfand".to_string(),
                        antragstext: "aint nobody got time for that".to_string(),
                        begründung: "Der Voltpfand muss dringend weggebracht werden.".to_string(),
                    }],
                },
                Top {
                    name: "Volt Zapfanlage".to_string(),
                    weight: 2,
                    top_type: TopType::Normal,
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
            datetime: NaiveDate::from_ymd_opt(2022, 5, 27)
                .unwrap()
                .and_hms_opt(7, 30, 15)
                .unwrap(),
            raete: vec![
                Rat {
                    name: "Valentin".to_string(),
                    abgemeldet: false,
                },
                Rat {
                    name: "Jonas \"Kooptimus\"".to_string(),
                    abgemeldet: false,
                },
                Rat {
                    name: "Marcel \"Markal\"".to_string(),
                    abgemeldet: false,
                },
                Rat {
                    name: "Elif".to_string(),
                    abgemeldet: true,
                },
                Rat {
                    name: "Australian".to_string(),
                    abgemeldet: true,
                },
            ],
            events: vec![],
            tops: vec![],
        };

        assert_eq!(template.render().unwrap(), PROTOKOLL_WITH_RÄTE);
    }

    // #[test]
    // randomly breaks because of timezone issues.. bad test
    fn render_with_events() {
        let template = ProtokollTemplate {
            datetime: NaiveDate::from_ymd_opt(2022, 5, 27)
                .unwrap()
                .and_hms_opt(7, 30, 15)
                .unwrap(),
            raete: vec![],
            events: vec![
                Event {
                    title: "Spieleabend".to_string(),
                    location: "33er".to_string(),
                    start: Utc.with_ymd_and_hms(2042, 4, 5, 17, 00, 00).unwrap().into(),
                },
                Event {
                    title: "Semestergrillen".to_string(),
                    location: "Grillplätze bei der Mathe".to_string(),
                    start: Utc
                        .with_ymd_and_hms(2042, 4, 12, 17, 00, 00)
                        .unwrap()
                        .into(),
                },
            ],
            tops: vec![],
        };

        assert_eq!(template.render().unwrap(), PROTOKOLL_WITH_EVENTS);
    }
}
