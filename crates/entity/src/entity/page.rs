use std::collections::BTreeSet;

use crate::{PageContent, PageId, PageLink, PageLinkTo, PageTitle};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Page {
    id: PageId,
    content: PageContent,
}

impl Page {
    pub fn broken_links(&self) -> BTreeSet<PageTitle> {
        self.content.broken_links()
    }

    pub fn content(&self) -> &PageContent {
        &self.content
    }

    pub fn id(&self) -> &PageId {
        &self.id
    }

    pub fn new(id: PageId, content: PageContent) -> Self {
        Self { id, content }
    }

    pub fn title(&self) -> PageTitle {
        self.content.title()
    }

    // TODO: pub fn rev_page_links(&self) -> BTreeSet<PageId>; // PageId -> PageGraph -> BTreeSet<PageId>
    // TODO: pub fn rev_title_links(&self) -> BTreeSet<PageId>; // PageTitle -> PageGraph -> BTreeSet<PageId>

    pub fn title_links(&self) -> Vec<PageLink> {
        self.content
            .title_links()
            .into_iter()
            .map(|page_title| PageLink::new(self.id, PageLinkTo::Title(page_title)))
            .collect::<Vec<PageLink>>()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{PageContent, PageId, PageTitle};

    use super::*;

    #[test]
    fn broken_links_test() -> anyhow::Result<()> {
        let id = PageId::from_str("20210203T040506Z")?;
        let content = PageContent::from("# title1\n\n[foo] [bar]".to_string());
        let page = Page::new(id, content);
        assert_eq!(page.broken_links(), {
            let mut set = BTreeSet::new();
            set.insert(PageTitle::from("foo".to_string()));
            set.insert(PageTitle::from("bar".to_string()));
            set
        });
        Ok(())
    }

    #[test]
    fn new_test() -> anyhow::Result<()> {
        let id = PageId::from_str("20210203T040506Z")?;
        let content = PageContent::from("# title1\n\ncontent1".to_string());
        let page = Page::new(id, content.clone());
        assert_eq!(page.id(), &id);
        assert_eq!(page.title(), PageTitle::from("title1".to_string()));
        assert_eq!(page.content(), &content);
        Ok(())
    }

    #[test]
    fn title_links_test() -> anyhow::Result<()> {
        let id = PageId::from_str("20210203T040506Z")?;
        let content = PageContent::from(
            vec![
                "# title1",
                "",
                "content1",
                "",
                "[title1]",
                "[title2]",
                "",
                "[title1]: /titles/title1",
                "[title2]: /titles/title2",
            ]
            .join("\n"),
        );
        let page = Page::new(id, content);
        assert_eq!(
            page.title_links(),
            vec![
                PageLink::new(id, PageLinkTo::Title(PageTitle::from("title1".to_string()))),
                PageLink::new(id, PageLinkTo::Title(PageTitle::from("title2".to_string())))
            ]
        );
        Ok(())
    }
}
