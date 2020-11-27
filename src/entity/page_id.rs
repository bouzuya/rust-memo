use chrono::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PageId(i64);

impl PageId {
  pub fn new() -> Option<Self> {
    Self::from_timestamp(Utc::now().timestamp())
  }

  // "YYYYMMDDTHHMMSSZ.md"
  // "http://localhost:3000/pages/YYYYMMDDTHHMMSSZ"
  pub fn from_like_str(s: &str) -> Option<Self> {
    use regex::Regex;
    let re = Regex::new(r"^(?:.*)(\d{4}\d{2}\d{2}T\d{2}\d{2}\d{2}Z)(?:.*)$").unwrap();
    re.captures(s)
      .and_then(|captures| Self::from_str(captures.get(1).unwrap().as_str()))
  }

  pub fn from_str(s: &str) -> Option<Self> {
    NaiveDateTime::parse_from_str(s, "%Y%m%dT%H%M%SZ")
      .ok()
      .map(|naive_date_time| DateTime::<Utc>::from_utc(naive_date_time, Utc).timestamp())
      .map(|timestamp| Self::from_timestamp(timestamp))
      .flatten()
  }

  pub fn from_timestamp(timestamp: i64) -> Option<Self> {
    if (0..=Utc.ymd(2999, 12, 31).and_hms(23, 59, 59).timestamp()).contains(&timestamp) {
      Some(PageId(timestamp))
    } else {
      None
    }
  }

  pub fn to_string(&self) -> String {
    let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.0, 0), Utc);
    format!("{}", dt.format("%Y%m%dT%H%M%SZ"))
  }
}

mod test {
  #[test]
  fn new_test() {
    assert_ne!(
      super::PageId::new(),
      super::PageId::from_str("20200808T101010Z")
    );
  }

  #[test]
  fn from_test() {
    let s = "20200808T002147Z";
    let d = 1596846107;
    let from_d = super::PageId::from_timestamp(d).unwrap();
    let from_s = super::PageId::from_str(s).unwrap();
    assert_eq!(from_d, from_s);
    assert_eq!(from_d.to_string(), s);
    assert_eq!(from_s.to_string(), s);

    assert_eq!(super::PageId::from_timestamp(32503680000), None);
    assert_eq!(super::PageId::from_str("30000101T000000Z"), None);
  }

  #[test]
  fn from_like_str_test() {
    let s = "20200808T002147Z";
    let from_s = super::PageId::from_str(s).unwrap();
    let like1 = super::PageId::from_like_str("20200808T002147Z.md").unwrap();
    let like2 =
      super::PageId::from_like_str("http://localhost:3000/pages/20200808T002147Z").unwrap();
    assert_eq!(from_s, like1);
    assert_eq!(from_s, like2);
  }
}
