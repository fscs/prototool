use askama::Template;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Antrag {
    pub titel: String,
    pub antragstext: String,
    pub begründung: String,
}

#[derive(Debug, Deserialize)]
struct Top {
    pub position: i64,
    pub name: String,
    pub anträge: Vec<Antrag>,
}

#[derive(Debug, Template)]
#[template(path = "../templates/protokoll.md")]
struct ProtokollTemplate {
    pub tops: Vec<Top>,
    pub date: String,
    pub date_machine: String,
}

mod filters {
    pub fn length<T>(input: &Vec<T>) -> Result<usize, askama::Error>
    where
        T: Sized,
    {
        Ok(input.len())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{Antrag, ProtokollTemplate, Top};
    use askama::Template;
    use pretty_assertions::assert_eq;

    static PROTOKOLL_NO_TOPS: &'static str = include_str!("../../tests/protokoll-no-tops.md");
    static PROTOKOLL_WITH_TOPS: &'static str = include_str!("../../tests/protokoll-with-tops.md");

    #[test]
    fn render_without_tops() {
        let template = ProtokollTemplate {
            date: "18.5.2022".to_string(),
            date_machine: "2022-05-27T07:30:15.000Z".to_string(),
            tops: vec![],
        };

        assert_eq!(template.render().unwrap(), PROTOKOLL_NO_TOPS);
    }

    #[test]
    fn render_with_tops() {
        let template = ProtokollTemplate {
            date: "18.5.2022".to_string(),
            date_machine: "2022-05-27T07:30:15.000Z".to_string(),
            tops: vec![
                Top {
                    name: "FZB: 25€ Blumen für Valentin".to_string(),
                    position: 1,
                    anträge: vec![Antrag {
                        titel: "Blumen für Valentin".to_string(),
                        antragstext: "Wir möchten Blumen für Valentin kaufen".to_string(),
                        begründung: "Weil wir Valentin toll finden".to_string(),
                    }],
                },
                Top {
                    name: "Volt Zapfanlage".to_string(),
                    position: 2,
                    anträge: vec![
                        Antrag {
                            titel: "Tank für Voltzapfanlage".to_string(),
                            antragstext: "Der Tank soll im Keller installiert werden. ".to_string(),
                            begründung: "Volt aus dem Hahn > Volt aus der Dose".to_string(),
                        },
                        Antrag {
                            titel: "Hahn für Voltzapfanlage".to_string(),
                            antragstext: "Der Hahn soll beim Telefon angebracht werden".to_string(),
                            begründung: "Volt aus dem Hahn > Volt aus der Dose".to_string(),
                        },
                    ],
                },
            ],
        };

        assert_eq!(template.render().unwrap(), PROTOKOLL_WITH_TOPS);
    }
}
