use crate::helpers;
use crate::url_helpers::title_url;
use anyhow::Context;
use entity::{PageId, PageTitle};
use pulldown_cmark::{BrokenLink, Options, Parser};
use std::{collections::BTreeSet, fs, str::FromStr};

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

pub fn insert_links(id_like: &str) -> anyhow::Result<()> {
    let page_id = PageId::from_like_str(id_like).context("page_id parse failed")?;
    let file_name = helpers::to_file_name(&page_id);
    let mut content = fs::read_to_string(&file_name)?;
    let links = broken_links(&content);
    if !links.is_empty() {
        content.push('\n');
    }
    for link in links {
        let page_title = PageTitle::from_str(link.as_str())?;
        let url = title_url(&page_title);
        content.push_str(&format!("[{}]: {}", link, url));
    }
    fs::write(file_name, content)?;
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
