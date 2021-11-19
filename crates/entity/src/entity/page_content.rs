use std::collections::BTreeSet;

use pulldown_cmark::{BrokenLink, Options, Parser};

use crate::{PageId, PagePath};

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct PageContent(String);

fn broken_links_impl(content: &str) -> BTreeSet<String> {
    let mut res = BTreeSet::new();
    let mut callback = |broken_link: BrokenLink| {
        res.insert(broken_link.reference.to_owned());
        None
    };
    let parser =
        Parser::new_with_broken_link_callback(content, Options::empty(), Some(&mut callback));
    for _ in parser {}
    res
}

impl PageContent {
    pub fn broken_links(&self) -> BTreeSet<String> {
        broken_links_impl(self.0.as_str())
    }

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
    use std::{iter::FromIterator, str::FromStr};

    use super::*;

    #[test]
    fn broken_links_impl_test() {
        let set = |s: &[&str]| -> BTreeSet<String> {
            BTreeSet::from_iter(s.iter().map(|i| i.to_string()))
        };
        let f = broken_links_impl;

        assert!(f("").is_empty());

        assert_eq!(f("[foo]"), set(&["foo"]));
        assert_eq!(f("[foo bar]"), set(&["foo bar"]));
        assert_eq!(f("[foo \"bar\"]"), set(&["foo \"bar\""]));

        assert!(f("[foo]\n\n[foo]: xxx").is_empty());
        assert_eq!(f("[foo]\n[foo]: url"), set(&["foo"]));
        assert_eq!(f("[foo]\n[foo]: url\n"), set(&["foo"]));

        assert_eq!(f("[foo] [bar]"), set(&["foo", "bar"]));
        assert_eq!(f("[foo]\n[bar]"), set(&["foo", "bar"]));
        assert_eq!(f("[foo]\n\n[bar]"), set(&["foo", "bar"]));
        assert_eq!(f("[foo] [foo]"), set(&["foo"]));

        assert!(f("[foo]()").is_empty());
        assert!(f("[](url)").is_empty());
        assert!(f("[]").is_empty());
    }

    #[test]
    fn broken_links_test() -> anyhow::Result<()> {
        let page_content = PageContent::from(
            vec![
                "# title1",
                "",
                "[link1]",
                "",
                "[link2] [link3]",
                "",
                "[link1]",
                "",
                "",
            ]
            .join("\n"),
        );
        assert_eq!(
            page_content.broken_links(),
            BTreeSet::from_iter(["link1", "link2", "link3"].iter().map(|x| x.to_string()))
        );
        Ok(())
    }

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
