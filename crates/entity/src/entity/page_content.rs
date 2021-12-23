use std::{collections::BTreeSet, str::FromStr};

use pulldown_cmark::{BrokenLink, Options, Parser};
use regex::Regex;

use crate::{PageId, PagePath, PageTitle, TitlePath};

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct PageContent(String);

fn broken_links(content: &str) -> BTreeSet<String> {
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
    pub fn broken_links(&self) -> BTreeSet<PageTitle> {
        let links = broken_links(self.0.as_str());
        links
            .into_iter()
            .map(PageTitle::from)
            .collect::<BTreeSet<PageTitle>>()
    }

    pub fn ensure_links(&mut self) {
        let links = broken_links(self.0.as_str());
        if links.is_empty() {
            return;
        }
        self.0.push('\n');
        self.0.push_str(
            links
                .into_iter()
                .map(|link| -> String {
                    let page_title = PageTitle::from(link.clone());
                    let url = TitlePath::from(page_title).to_string();
                    format!("[{}]: {}", link, url)
                })
                .collect::<Vec<String>>()
                .join("\n")
                .as_str(),
        );
        self.0.push('\n');
    }

    pub fn obsoletes(&self) -> Vec<PageId> {
        self.0
            .find("\n## Obsoletes")
            .map(|index| {
                let regex =
                    Regex::new(r"^- \[(\d{4}\d{2}\d{2}T\d{2}\d{2}\d{2}Z)\]\(.*\)$").unwrap();
                self.0[index..]
                    .lines()
                    .filter_map(|line| {
                        regex
                            .captures(line)
                            .and_then(|captures| captures.get(1))
                            .and_then(|m| PageId::from_str(m.as_str()).ok())
                    })
                    .collect::<Vec<PageId>>()
            })
            .unwrap_or_default()
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

    pub fn title(&self) -> PageTitle {
        self.0
            .lines()
            .next()
            .and_then(|first_line| first_line.strip_prefix("# "))
            .map(|s| s.to_string())
            .map(PageTitle::from)
            .unwrap_or_default()
    }

    pub fn title_links(&self) -> Vec<PageTitle> {
        pulldown_cmark::Parser::new(self.0.as_str())
            .filter_map(|event| match event {
                pulldown_cmark::Event::End(tag) => Some(tag),
                _ => None,
            })
            .filter_map(|tag| match tag {
                pulldown_cmark::Tag::Link(_, to, _) => Some(to),
                _ => None,
            })
            .filter_map(|to| TitlePath::from_str(to.as_ref()).map(PageTitle::from).ok())
            .collect::<Vec<PageTitle>>()
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
    fn broken_links_test() {
        assert_eq!(
            PageContent::from("[foo] [bar]".to_string()).broken_links(),
            {
                let mut set = BTreeSet::new();
                set.insert(PageTitle::from("foo".to_string()));
                set.insert(PageTitle::from("bar".to_string()));
                set
            }
        );
    }

    #[test]
    fn broken_links_impl_test() {
        let set = |s: &[&str]| -> BTreeSet<String> {
            BTreeSet::from_iter(s.iter().map(|i| i.to_string()))
        };
        let f = broken_links;

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
    fn ensure_links_test() -> anyhow::Result<()> {
        let mut page_content = PageContent::from(
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
        page_content.ensure_links();
        assert_eq!(
            String::from(page_content),
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
                "[link1]: /titles/link1",
                "[link2]: /titles/link2",
                "[link3]: /titles/link3",
                "",
            ]
            .join("\n"),
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
    fn obsoletes_test() -> anyhow::Result<()> {
        let page_content = PageContent::from(vec!["# title1", "", "content1", ""].join("\n"));
        assert_eq!(page_content.obsoletes(), vec![]);

        let page_content = PageContent::from(
            vec![
                "# title1",
                "",
                "content1",
                "",
                "## Obsoletes",
                "",
                "- [20210203T040506Z](/pages/20210203T040506Z)",
                "- [20210203T040507Z](/pages/20210203T040507Z)",
                "",
            ]
            .join("\n"),
        );
        assert_eq!(
            page_content.obsoletes(),
            vec![
                PageId::from_str("20210203T040506Z")?,
                PageId::from_str("20210203T040507Z")?,
            ]
        );
        Ok(())
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
    fn title_test() -> anyhow::Result<()> {
        let page_content = PageContent::from(vec!["# title1", "", "content1"].join("\n"));
        assert_eq!(page_content.title(), PageTitle::from("title1".to_string()));
        let page_content = PageContent::from(vec!["foo"].join("\n"));
        assert_eq!(page_content.title(), PageTitle::default());
        Ok(())
    }

    #[test]
    fn title_links_test() {
        let page_content = PageContent::from(
            vec![
                "# title1",
                "",
                "content1",
                "",
                "[title1]",
                "",
                "[title2] [title3]",
                "",
                "[20210203T040506Z]",
                "",
                "[inline](/titles/bouzuya)",
                "[ref][ref]",
                "[ref_unknown][ref_unknown]",
                "[collapsed][]",
                "[collapsed_unknown][]",
                "[shortcut]",
                "[shortcut_unknown]",
                "</titles/%E3%81%BC%E3%81%86%E3%81%9A%E3%82%844>",
                "",
                "[ref]: /titles/%E3%81%BC%E3%81%86%E3%81%9A%E3%82%841",
                "[collapsed]: /titles/%E3%81%BC%E3%81%86%E3%81%9A%E3%82%842",
                "[shortcut]: /titles/%E3%81%BC%E3%81%86%E3%81%9A%E3%82%843",
                "",
                "[title1]: /titles/title1",
                "[title2]: /titles/title2",
                "[title3]: /titles/title3",
                "[20210203T040506Z]: /pages/20210203T040506Z",
                "",
            ]
            .join("\n"),
        );
        assert_eq!(
            page_content.title_links(),
            vec![
                "title1",
                "title2",
                "title3",
                "bouzuya",
                "ぼうずや1",
                "ぼうずや2",
                "ぼうずや3"
            ]
            .iter()
            .map(|s| s.to_string())
            .map(PageTitle::from)
            .collect::<Vec<PageTitle>>()
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
