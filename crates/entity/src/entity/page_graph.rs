use std::collections::{BTreeMap, BTreeSet};

use crate::{Page, PageId, PageTitle};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PageGraph {
    obsolete_links: BTreeMap<PageId, BTreeSet<PageId>>,
    rev_obsolete_links: BTreeMap<PageId, BTreeSet<PageId>>,
    titles: BTreeMap<PageId, PageTitle>,
    rev_titles: BTreeMap<PageTitle, BTreeSet<PageId>>,
    rev_title_links: BTreeMap<PageTitle, BTreeSet<PageId>>,
}

impl PageGraph {
    pub fn add_page(&mut self, page: Page) {
        let page_id = *page.id();
        let mut page_content = page.content().clone();
        page_content.ensure_links();
        for obsoleted in page_content.obsoletes() {
            self.obsolete_links
                .entry(page_id)
                .or_insert_with(BTreeSet::new)
                .insert(obsoleted);
            self.rev_obsolete_links
                .entry(obsoleted)
                .or_insert_with(BTreeSet::new)
                .insert(page_id);
        }
        let page_title = page.title();
        self.titles.insert(page_id, page_title.clone());
        self.rev_titles
            .entry(page_title)
            .or_insert_with(BTreeSet::new)
            .insert(page_id);
        let title_links = page_content.title_links();
        for linked_page_title in title_links {
            self.rev_title_links
                .entry(linked_page_title)
                .or_insert_with(BTreeSet::new)
                .insert(page_id);
        }
    }

    pub fn is_obsoleted(&self, page_id: &PageId) -> bool {
        self.rev_obsolete_links
            .get(page_id)
            .map(|m| !m.is_empty())
            .unwrap_or_default()
    }

    pub fn obsoleted_by(&self, page_id: &PageId) -> BTreeSet<PageId> {
        self.rev_obsolete_links
            .get(page_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn obsoletes(&self, page_id: &PageId) -> BTreeSet<PageId> {
        self.obsolete_links
            .get(page_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn title(&self, page_id: &PageId) -> Option<PageTitle> {
        self.titles.get(page_id).cloned()
    }

    pub fn titles(&self) -> BTreeSet<PageTitle> {
        self.titles
            .values()
            .cloned()
            .collect::<BTreeSet<PageTitle>>()
    }

    pub fn titled(&self, page_title: &PageTitle) -> BTreeSet<PageId> {
        self.rev_titles.get(page_title).cloned().unwrap_or_default()
    }

    pub fn find_ids_link_to(&self, page_title: &PageTitle) -> BTreeSet<PageId> {
        self.rev_title_links
            .get(page_title)
            .cloned()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::PageContent;

    use super::*;

    #[test]
    fn obsoleted_by_test() -> anyhow::Result<()> {
        let page_id1 = PageId::from_str("20210203T040506Z")?;
        let page_id2 = PageId::from_str("20210203T040507Z")?;
        let page_id3 = PageId::from_str("20210203T040508Z")?;
        let page_content2 = PageContent::from(
            vec![
                "# title2",
                "",
                "## Obsoletes",
                "",
                "- [20210203T040506Z](/pages/20210203T040506Z)",
                "",
            ]
            .join("\n"),
        );
        let page_content3 = PageContent::from(
            vec![
                "# title3",
                "",
                "## Obsoletes",
                "",
                "- [20210203T040506Z](/pages/20210203T040506Z)",
                "",
            ]
            .join("\n"),
        );

        let page_graph = PageGraph::default();
        assert!(!page_graph.is_obsoleted(&page_id1));
        assert!(page_graph.obsoleted_by(&page_id1).is_empty());
        assert!(page_graph.obsoletes(&page_id1).is_empty());

        let mut page_graph = PageGraph::default();
        page_graph.add_page(Page::new(page_id2, page_content2));
        page_graph.add_page(Page::new(page_id3, page_content3));
        assert!(page_graph.is_obsoleted(&page_id1));
        assert!(!page_graph.is_obsoleted(&page_id2));
        assert!(!page_graph.is_obsoleted(&page_id3));
        assert!(page_graph.obsoletes(&page_id1).is_empty());
        assert_eq!(
            page_graph.obsoletes(&page_id2),
            vec![page_id1].into_iter().collect::<BTreeSet<_>>()
        );
        assert_eq!(
            page_graph.obsoletes(&page_id3),
            vec![page_id1].into_iter().collect::<BTreeSet<_>>()
        );
        assert_eq!(
            page_graph.obsoleted_by(&page_id1),
            vec![page_id2, page_id3]
                .into_iter()
                .collect::<BTreeSet<_>>()
        );
        assert!(page_graph.obsoleted_by(&page_id2).is_empty());
        assert!(page_graph.obsoleted_by(&page_id3).is_empty());
        Ok(())
    }

    #[test]
    fn titled_test() -> anyhow::Result<()> {
        let page_id1 = PageId::from_str("20210203T040506Z")?;
        let page_id2 = PageId::from_str("20210203T040507Z")?;
        let page_id3 = PageId::from_str("20210203T040508Z")?;
        let page_content1 = PageContent::from("# title1".to_string());
        let page_content2 = PageContent::from("# title1".to_string());
        let page_content3 = PageContent::from("# title3".to_string());
        let page_title1 = PageTitle::from("title1".to_string());
        let page_title3 = PageTitle::from("title3".to_string());

        let page_graph = PageGraph::default();
        assert!(page_graph.title(&page_id1).is_none());
        assert!(page_graph.titled(&page_title1).is_empty());

        let mut page_graph = PageGraph::default();
        page_graph.add_page(Page::new(page_id1, page_content1));
        page_graph.add_page(Page::new(page_id2, page_content2));
        page_graph.add_page(Page::new(page_id3, page_content3));
        assert_eq!(page_graph.title(&page_id1), Some(page_title1.clone()));
        assert_eq!(
            page_graph.titled(&page_title1),
            vec![page_id1, page_id2]
                .into_iter()
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(page_graph.title(&page_id3), Some(page_title3.clone()));
        assert_eq!(
            page_graph.titled(&page_title3),
            vec![page_id3].into_iter().collect::<BTreeSet<_>>()
        );
        Ok(())
    }

    #[test]
    fn titles_test() -> anyhow::Result<()> {
        let page_id1 = PageId::from_str("20210203T040506Z")?;
        let page_id2 = PageId::from_str("20210203T040507Z")?;
        let page_id3 = PageId::from_str("20210203T040508Z")?;
        let page_content1 = PageContent::from("# title1".to_string());
        let page_content2 = PageContent::from("# title1".to_string());
        let page_content3 = PageContent::from("# title3".to_string());
        let page_title1 = PageTitle::from("title1".to_string());
        let page_title3 = PageTitle::from("title3".to_string());

        let page_graph = PageGraph::default();
        assert!(page_graph.titles().is_empty());

        let mut page_graph = PageGraph::default();
        page_graph.add_page(Page::new(page_id1, page_content1));
        page_graph.add_page(Page::new(page_id2, page_content2));
        page_graph.add_page(Page::new(page_id3, page_content3));
        assert_eq!(
            page_graph.titles(),
            vec![page_title1, page_title3]
                .into_iter()
                .collect::<BTreeSet<_>>()
        );
        Ok(())
    }

    #[test]
    fn find_ids_link_to_test() -> anyhow::Result<()> {
        let page_id1 = PageId::from_str("20210203T040506Z")?;
        let page_id2 = PageId::from_str("20210203T040507Z")?;
        let page_id3 = PageId::from_str("20210203T040508Z")?;
        let page_content1 = PageContent::from("# title1\n[](/titles/title3)".to_string());
        let page_content2 = PageContent::from("# title1".to_string());
        let page_content3 = PageContent::from("# title3\n[](/titles/title1)".to_string());
        let page_title1 = PageTitle::from("title1".to_string());
        let page_title3 = PageTitle::from("title3".to_string());

        let page_graph = PageGraph::default();
        assert!(page_graph.find_ids_link_to(&page_title1).is_empty());

        let mut page_graph = PageGraph::default();
        page_graph.add_page(Page::new(page_id1, page_content1));
        page_graph.add_page(Page::new(page_id2, page_content2));
        page_graph.add_page(Page::new(page_id3, page_content3));
        assert_eq!(
            page_graph.find_ids_link_to(&page_title1),
            vec![page_id3].into_iter().collect::<BTreeSet<_>>()
        );
        assert_eq!(
            page_graph.find_ids_link_to(&page_title3),
            vec![page_id1].into_iter().collect::<BTreeSet<_>>()
        );
        Ok(())
    }
}
