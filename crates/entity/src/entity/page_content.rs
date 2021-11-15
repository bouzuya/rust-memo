#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct PageContent(String);

impl std::fmt::Display for PageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for PageContent {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<PageContent> for String {
    fn from(page_content: PageContent) -> Self {
        page_content.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_test() {
        assert_eq!(String::from(PageContent::default()), "");
    }

    #[test]
    fn display_test() {
        assert_eq!(
            PageContent::from("content1".to_string()).to_string(),
            "content1"
        );
    }

    #[test]
    fn str_conversion_test() {
        assert_eq!(
            String::from(PageContent::from("content1".to_string())),
            "content1"
        );
    }
}
