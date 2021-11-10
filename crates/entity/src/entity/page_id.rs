use chrono::prelude::*;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("parse page id error")]
pub struct ParsePageIdError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PageId(i64);

impl PageId {
    pub fn new() -> Option<Self> {
        Self::from_timestamp(Utc::now().timestamp())
    }

    // "YYYYMMDDTHHMMSSZ.md"
    // "http://localhost:3000/pages/YYYYMMDDTHHMMSSZ"
    pub fn from_like_str(s: &str) -> Result<Self, ParsePageIdError> {
        use regex::Regex;
        let re = Regex::new(r"^(?:.*)(\d{4}\d{2}\d{2}T\d{2}\d{2}\d{2}Z)(?:.*)$").unwrap();
        let captures = re.captures(s).ok_or(ParsePageIdError)?;
        let capture = captures.get(1).ok_or(ParsePageIdError)?;
        Self::from_str(capture.as_str())
    }

    pub fn from_timestamp(timestamp: i64) -> Option<Self> {
        if (0..=Utc.ymd(2999, 12, 31).and_hms(23, 59, 59).timestamp()).contains(&timestamp) {
            Some(PageId(timestamp))
        } else {
            None
        }
    }
}

impl std::fmt::Display for PageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.0, 0), Utc);
        write!(f, "{}", dt.format("%Y%m%dT%H%M%SZ"))
    }
}

impl std::str::FromStr for PageId {
    type Err = ParsePageIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let naive_date_time =
            NaiveDateTime::parse_from_str(s, "%Y%m%dT%H%M%SZ").map_err(|_| ParsePageIdError)?;
        let timestamp = DateTime::<Utc>::from_utc(naive_date_time, Utc).timestamp();
        Self::from_timestamp(timestamp).ok_or(ParsePageIdError)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn new_test() -> anyhow::Result<()> {
        // TODO: unwrap
        assert_ne!(
            PageId::new().unwrap(),
            PageId::from_str("20200808T101010Z")?
        );
        Ok(())
    }

    #[test]
    fn from_test() -> anyhow::Result<()> {
        let s = "20200808T002147Z";
        let d = 1596846107;
        // TODO: unwrap
        let from_d = PageId::from_timestamp(d).unwrap();
        let from_s = PageId::from_str(s)?;
        assert_eq!(from_d, from_s);
        assert_eq!(from_d.to_string(), s);
        assert_eq!(from_s.to_string(), s);

        assert_eq!(PageId::from_timestamp(32503680000), None);
        assert!(PageId::from_str("30000101T000000Z").is_err());
        Ok(())
    }

    #[test]
    fn from_like_str_test() -> anyhow::Result<()> {
        let s = "20200808T002147Z";
        let from_s = PageId::from_str(s)?;
        let like1 = PageId::from_like_str("20200808T002147Z.md")?;
        let like2 = PageId::from_like_str("http://localhost:3000/pages/20200808T002147Z")?;
        assert_eq!(from_s, like1);
        assert_eq!(from_s, like2);
        Ok(())
    }
}
