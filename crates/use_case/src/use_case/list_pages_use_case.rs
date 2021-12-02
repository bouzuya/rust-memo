use entity::PageId;

use crate::{HasPageRepository, PageRepository};

pub trait ListPagesUseCase: HasPageRepository {
    fn list_pages(&self, all: bool) -> anyhow::Result<Vec<(PageId, bool)>> {
        let page_graph = self.page_repository().load_page_graph()?;
        let mut page_ids = self.page_repository().find_ids()?;
        page_ids.reverse();
        let pages = page_ids
            .into_iter()
            .map(|page_id| (page_id, page_graph.is_obsoleted(&page_id)))
            .filter(|(_, obsoleted)| all || !obsoleted)
            .collect::<Vec<(PageId, bool)>>();
        Ok(pages)
    }
}

impl<T: HasPageRepository> ListPagesUseCase for T {}

pub trait HasListPagesUseCase {
    type ListPagesUseCase: ListPagesUseCase;

    fn list_pages_use_case(&self) -> &Self::ListPagesUseCase;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use entity::{Page, PageContent, PageGraph, PageId};

    use super::*;
    use crate::MockPageRepository;

    struct TestApp {
        page_repository: MockPageRepository,
    }

    impl HasPageRepository for TestApp {
        type PageRepository = MockPageRepository;

        fn page_repository(&self) -> &Self::PageRepository {
            &self.page_repository
        }
    }

    impl HasListPagesUseCase for TestApp {
        type ListPagesUseCase = TestApp;

        fn list_pages_use_case(&self) -> &Self::ListPagesUseCase {
            self
        }
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        {
            let mut page_repository = MockPageRepository::new();
            page_repository
                .expect_load_page_graph()
                .returning(|| Ok(PageGraph::default()));
            page_repository.expect_find_ids().returning(|| Ok(vec![]));
            let app = TestApp { page_repository };
            let pages = app.list_pages_use_case().list_pages(true)?;
            assert!(pages.is_empty());
        }

        {
            let mut page_repository = MockPageRepository::new();
            let page_id1 = PageId::from_str("20210203T040506Z")?;
            let page_id2 = PageId::from_str("20210203T040507Z")?;
            page_repository.expect_load_page_graph().returning(move || {
                let mut page_graph = PageGraph::default();
                page_graph.add_page({
                    let page_content = PageContent::from(vec!["# title1"].join("\n"));
                    Page::new(page_id1, page_content)
                });
                page_graph.add_page({
                    let page_content = PageContent::from(
                        vec![
                            "# title2",
                            "## Obsoletes",
                            "",
                            "- [20210203T040506Z](/pages/20210203T040506Z)",
                            "",
                        ]
                        .join("\n"),
                    );
                    Page::new(page_id2, page_content)
                });
                Ok(page_graph)
            });
            page_repository
                .expect_find_ids()
                .returning(move || Ok(vec![page_id1, page_id2]));
            let app = TestApp { page_repository };
            let pages = app.list_pages_use_case().list_pages(false)?;
            assert_eq!(pages, vec![(page_id2, false)]);
            let pages = app.list_pages_use_case().list_pages(true)?;
            assert_eq!(pages, vec![(page_id2, false), (page_id1, true)]);
        }
        Ok(())
    }
}
