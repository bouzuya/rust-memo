#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PageTitle(String);

impl PageTitle {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn empty() -> Self {
        Self::from_str("")
    }

    pub fn from_str(s: &str) -> Self {
        PageTitle(s.to_owned())
    }
}

impl std::fmt::Display for PageTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_test() {
        let page_title1 = PageTitle::empty();
        let page_title2 = PageTitle::from_str("");
        assert_eq!(page_title1, page_title2);
    }

    #[test]
    fn from_eq_test() {
        let s = "title1";
        let page_title1 = PageTitle::from_str(s);
        let page_title2 = PageTitle::from_str(s);
        assert_eq!(page_title1.to_string(), s.to_owned());
        assert_eq!(page_title2.to_string(), s.to_owned());
        assert_eq!(page_title1, page_title2);
    }
}
