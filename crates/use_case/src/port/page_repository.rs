use entity::{LineNumber, Page, PageContent, PageGraph, PageId, Query};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait PageRepository {
    // TODO: add tests
    // TODO: add ColumnNumber
    fn find_by_query(&self, query: &Query) -> anyhow::Result<Vec<(PageId, LineNumber, usize)>> {
        let mut res = vec![];
        for page_id in self.find_ids()? {
            let page_content = self.find_content(&page_id)?;
            if let Some(page_content) = page_content {
                res.extend(
                    query
                        .matches(String::from(page_content).as_str())
                        .into_iter()
                        .map(|(l, c)| (page_id, l, c)),
                );
            }
        }
        Ok(res)
    }

    fn find_content(&self, page_id: &PageId) -> anyhow::Result<Option<PageContent>>;

    fn find_ids(&self) -> anyhow::Result<Vec<PageId>>;

    fn load_page_graph(&self) -> anyhow::Result<PageGraph> {
        let mut page_graph = PageGraph::default();
        for page_id in self.find_ids()? {
            if let Some(page_content) = self.find_content(&page_id)? {
                let page = Page::new(page_id, page_content);
                page_graph.add_page(page);
            }
        }
        Ok(page_graph)
    }

    fn save_content(&self, page_id: &PageId, content: PageContent) -> anyhow::Result<()>;
}

pub trait HasPageRepository {
    type PageRepository: PageRepository;

    fn page_repository(&self) -> &Self::PageRepository;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn load_page_graph_test() -> anyhow::Result<()> {
        struct TestRepository {}
        impl PageRepository for TestRepository {
            fn find_content(&self, page_id: &PageId) -> anyhow::Result<Option<PageContent>> {
                let page_id1 = PageId::from_str("20210203T040506Z")?;
                let page_id2 = PageId::from_str("20210203T040507Z")?;
                let page_content1 = PageContent::from("# title1".to_string());
                let page_content2 = PageContent::from(
                    vec![
                        "# title1",
                        "",
                        "## Obsoletes",
                        "",
                        "- [20210203T040506Z](/pages/20210203T040506Z)",
                    ]
                    .join("\n"),
                );
                if page_id == &page_id1 {
                    Ok(Some(page_content1))
                } else if page_id == &page_id2 {
                    Ok(Some(page_content2))
                } else {
                    unreachable!()
                }
            }

            fn find_ids(&self) -> anyhow::Result<Vec<PageId>> {
                let page_id1 = PageId::from_str("20210203T040506Z")?;
                let page_id2 = PageId::from_str("20210203T040507Z")?;
                Ok(vec![page_id1, page_id2])
            }

            fn save_content(&self, _: &PageId, _: PageContent) -> anyhow::Result<()> {
                unreachable!()
            }
        }
        let page_repository = TestRepository {};
        let mut expected = PageGraph::default();
        let page_id1 = PageId::from_str("20210203T040506Z")?;
        let page_content1 = PageContent::from(vec!["# title1"].join("\n"));
        let page_id2 = PageId::from_str("20210203T040507Z")?;
        let page_content2 = PageContent::from(
            vec![
                "# title1",
                "",
                "## Obsoletes",
                "",
                "- [20210203T040506Z](/pages/20210203T040506Z)",
                "",
            ]
            .join("\n"),
        );
        expected.add_page(Page::new(page_id1, page_content1));
        expected.add_page(Page::new(page_id2, page_content2));
        assert_eq!(page_repository.load_page_graph()?, expected);
        Ok(())
    }
}
