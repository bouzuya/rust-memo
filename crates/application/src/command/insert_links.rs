use crate::url_helpers::title_url;
use anyhow::{anyhow, Context};
use entity::{PageId, PageTitle};
use pulldown_cmark::{BrokenLink, Options, Parser};
use std::{collections::BTreeSet, str::FromStr};
use use_case::{HasRepository, Repository};

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

pub fn insert_links<App: HasRepository>(app: App, id_like: &str) -> anyhow::Result<()> {
    let page_id = PageId::from_like_str(id_like)?;
    let mut content = app
        .repository()
        .find_content(&page_id)?
        .with_context(|| anyhow!("file not found: {}", page_id))?;
    let links = broken_links(&content);
    if !links.is_empty() {
        content.push('\n');
    }
    content.push_str(
        links
            .into_iter()
            .map(|link| -> anyhow::Result<String> {
                let page_title = PageTitle::from_str(link.as_str())?;
                let url = title_url(&page_title);
                Ok(format!("[{}]: {}", link, url))
            })
            .collect::<anyhow::Result<Vec<String>>>()?
            .join("\n")
            .as_str(),
    );
    app.repository().save(&page_id, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use super::*;

    #[test]
    fn broken_links_test() {
        let set = |s: &[&str]| -> BTreeSet<String> {
            BTreeSet::from_iter(s.iter().map(|i| i.to_string()))
        };

        assert!(broken_links("").is_empty());

        assert_eq!(broken_links("[foo]"), set(&["foo"]));
        assert_eq!(broken_links("[foo bar]"), set(&["foo bar"]));
        assert_eq!(broken_links("[foo \"bar\"]"), set(&["foo \"bar\""]));

        assert!(broken_links("[foo]\n\n[foo]: xxx").is_empty());
        assert_eq!(broken_links("[foo]\n[foo]: url"), set(&["foo"]));
        assert_eq!(broken_links("[foo]\n[foo]: url\n"), set(&["foo"]));

        assert_eq!(broken_links("[foo] [bar]"), set(&["foo", "bar"]));
        assert_eq!(broken_links("[foo]\n[bar]"), set(&["foo", "bar"]));
        assert_eq!(broken_links("[foo]\n\n[bar]"), set(&["foo", "bar"]));
        assert_eq!(broken_links("[foo] [foo]"), set(&["foo"]));

        assert!(broken_links("[foo]()").is_empty());
        assert!(broken_links("[](url)").is_empty());
        assert!(broken_links("[]").is_empty());
    }
}
