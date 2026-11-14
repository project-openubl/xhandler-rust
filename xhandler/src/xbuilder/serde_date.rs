use chrono::NaiveDate;
use serde::{self, Deserialize, Deserializer};

const DD_MM_YYYY: &str = "%d-%m-%Y";
const ISO_8601: &str = "%Y-%m-%d";

pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<NaiveDate, D::Error> {
    let s = String::deserialize(d)?;
    NaiveDate::parse_from_str(&s, DD_MM_YYYY)
        .or_else(|_| NaiveDate::parse_from_str(&s, ISO_8601))
        .map_err(|_| {
            serde::de::Error::custom(format!(
                "fecha invalida: '{s}' (usar formato DD-MM-YYYY o YYYY-MM-DD)"
            ))
        })
}

pub mod option {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    use super::{DD_MM_YYYY, ISO_8601};

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<NaiveDate>, D::Error> {
        let opt: Option<String> = Option::deserialize(d)?;
        match opt {
            None => Ok(None),
            Some(s) => NaiveDate::parse_from_str(&s, DD_MM_YYYY)
                .or_else(|_| NaiveDate::parse_from_str(&s, ISO_8601))
                .map(Some)
                .map_err(|_| {
                    serde::de::Error::custom(format!(
                        "fecha invalida: '{s}' (usar formato DD-MM-YYYY o YYYY-MM-DD)"
                    ))
                }),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct TestDate {
        #[serde(deserialize_with = "super::deserialize")]
        date: NaiveDate,
    }

    #[derive(Deserialize)]
    struct TestOptionDate {
        #[serde(default, deserialize_with = "super::option::deserialize")]
        date: Option<NaiveDate>,
    }

    #[test]
    fn dd_mm_yyyy() {
        let json = r#"{"date": "15-03-2024"}"#;
        let t: TestDate = serde_json::from_str(json).unwrap();
        assert_eq!(t.date, NaiveDate::from_ymd_opt(2024, 3, 15).unwrap());
    }

    #[test]
    fn iso_fallback() {
        let json = r#"{"date": "2024-03-15"}"#;
        let t: TestDate = serde_json::from_str(json).unwrap();
        assert_eq!(t.date, NaiveDate::from_ymd_opt(2024, 3, 15).unwrap());
    }

    #[test]
    fn invalid_date() {
        let json = r#"{"date": "not-a-date"}"#;
        assert!(serde_json::from_str::<TestDate>(json).is_err());
    }

    #[test]
    fn option_dd_mm_yyyy() {
        let json = r#"{"date": "15-03-2024"}"#;
        let t: TestOptionDate = serde_json::from_str(json).unwrap();
        assert_eq!(t.date, Some(NaiveDate::from_ymd_opt(2024, 3, 15).unwrap()));
    }

    #[test]
    fn option_iso_fallback() {
        let json = r#"{"date": "2024-03-15"}"#;
        let t: TestOptionDate = serde_json::from_str(json).unwrap();
        assert_eq!(t.date, Some(NaiveDate::from_ymd_opt(2024, 3, 15).unwrap()));
    }

    #[test]
    fn option_null() {
        let json = r#"{"date": null}"#;
        let t: TestOptionDate = serde_json::from_str(json).unwrap();
        assert_eq!(t.date, None);
    }

    #[test]
    fn option_missing() {
        let json = r#"{}"#;
        let t: TestOptionDate = serde_json::from_str(json).unwrap();
        assert_eq!(t.date, None);
    }
}
