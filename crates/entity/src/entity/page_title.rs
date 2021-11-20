use thiserror::Error;

#[derive(Debug, Error)]
#[error("parse page title error")]
pub enum ParsePageTitleError {}

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct PageTitle(String);

impl PageTitle {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for PageTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PageTitle {
    type Err = ParsePageTitleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<String> for PageTitle {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<PageTitle> for String {
    fn from(page_title: PageTitle) -> Self {
        page_title.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_test() {
        let page_title1 = PageTitle::default();
        let page_title2 = PageTitle::from("".to_string());
        assert_eq!(page_title1, page_title2);
    }

    #[test]
    fn str_conversion_test() {
        assert_eq!(String::from(PageTitle::from("title".to_string())), "title");
    }

    #[test]
    fn from_eq_test() {
        let s = "title1";
        let page_title1 = PageTitle::from(s.to_string());
        let page_title2 = PageTitle::from(s.to_string());
        assert_eq!(page_title1.to_string(), s);
        assert_eq!(page_title2.to_string(), s);
        assert_eq!(page_title1, page_title2);
    }
}
