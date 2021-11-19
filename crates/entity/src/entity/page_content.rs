use crate::{PageId, PagePath};

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct PageContent(String);

impl PageContent {
    pub fn replace_obsoletes(&mut self, page_id: PageId) {
        if let Some(index) = self.0.find("\n## Obsoletes") {
            self.0.truncate(index);
        }
        self.0.push_str(&format!(
            "\n## Obsoletes\n\n- [{}]({})\n",
            page_id,
            PagePath::from(page_id),
        ));
    }
}

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
    use std::str::FromStr;

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
    fn replace_obsoletes_test() -> anyhow::Result<()> {
        let mut page_content = PageContent::from(vec!["# title1", "", "content1", ""].join("\n"));
        let page_id = PageId::from_str("20210203T040506Z")?;
        page_content.replace_obsoletes(page_id);
        assert_eq!(
            page_content.to_string(),
            vec![
                "# title1",
                "",
                "content1",
                "",
                "## Obsoletes",
                "",
                "- [20210203T040506Z](/pages/20210203T040506Z)",
                "",
            ]
            .join("\n"),
        );
        let page_id = PageId::from_str("20210203T040507Z")?;
        page_content.replace_obsoletes(page_id);
        assert_eq!(
            page_content.to_string(),
            vec![
                "# title1",
                "",
                "content1",
                "",
                "## Obsoletes",
                "",
                "- [20210203T040507Z](/pages/20210203T040507Z)",
                "",
            ]
            .join("\n"),
        );
        Ok(())
    }

    #[test]
    fn str_conversion_test() {
        assert_eq!(
            String::from(PageContent::from("content1".to_string())),
            "content1"
        );
    }
}
