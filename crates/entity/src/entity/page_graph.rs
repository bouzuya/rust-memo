use std::collections::{BTreeMap, BTreeSet};

use crate::{Page, PageId};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PageGraph {
    obsoletes: BTreeMap<PageId, BTreeSet<PageId>>,
    obsoleted_by: BTreeMap<PageId, BTreeSet<PageId>>,
}

impl PageGraph {
    pub fn add_page(&mut self, page: Page) {
        let page_id = *page.id();
        for obsoleted in page.content().obsoletes() {
            self.obsoletes
                .entry(page_id)
                .or_insert_with(BTreeSet::new)
                .insert(obsoleted);
            self.obsoleted_by
                .entry(obsoleted)
                .or_insert_with(BTreeSet::new)
                .insert(page_id);
        }
    }

    pub fn is_obsoleted(&self, page_id: &PageId) -> bool {
        self.obsoleted_by
            .get(page_id)
            .map(|m| !m.is_empty())
            .unwrap_or_default()
    }

    pub fn obsoleted_by(&self, page_id: &PageId) -> BTreeSet<PageId> {
        self.obsoleted_by.get(page_id).cloned().unwrap_or_default()
    }

    pub fn obsoletes(&self, page_id: &PageId) -> BTreeSet<PageId> {
        self.obsoletes.get(page_id).cloned().unwrap_or_default()
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
}
