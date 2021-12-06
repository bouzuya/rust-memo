use std::cmp::Reverse;

use entity::PageTitle;

use crate::{HasPageRepository, PageRepository};

pub trait ListTitlesUseCase: HasPageRepository {
    fn list_titles(&self, all: bool) -> anyhow::Result<Vec<(PageTitle, bool)>> {
        let page_graph = self.page_repository().load_page_graph()?;

        let mut title_items = vec![];
        for page_title in page_graph.titles() {
            let page_ids = page_graph.titled(&page_title);
            let obsoleted = !page_ids
                .iter()
                .any(|page_id| !page_graph.is_obsoleted(page_id));
            if all || !obsoleted {
                let page_id = page_graph
                    .titled(&page_title)
                    .into_iter()
                    .rev()
                    .next()
                    .unwrap(); // TODO: unwrap
                title_items.push((page_title, obsoleted, page_id));
            }
        }
        // TODO: clone
        title_items.sort_by_key(|(t, o, i)| (Reverse(i.clone()), t.clone(), o.clone()));

        Ok(title_items
            .into_iter()
            .map(|(t, o, _)| (t, o))
            .collect::<Vec<_>>())
    }
}

impl<T: HasPageRepository> ListTitlesUseCase for T {}

pub trait HasListTitlesUseCase {
    type ListTitlesUseCase: ListTitlesUseCase;

    fn list_titles_use_case(&self) -> &Self::ListTitlesUseCase;
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

    impl HasListTitlesUseCase for TestApp {
        type ListTitlesUseCase = TestApp;

        fn list_titles_use_case(&self) -> &Self::ListTitlesUseCase {
            self
        }
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        let mut page_repository = MockPageRepository::new();
        page_repository
            .expect_load_page_graph()
            .returning(|| Ok(PageGraph::default()));
        let app = TestApp { page_repository };
        let titles = app.list_titles_use_case().list_titles(true)?;
        assert!(titles.is_empty());
        Ok(())
    }

    #[test]
    fn test2() -> anyhow::Result<()> {
        let mut page_repository = MockPageRepository::new();
        page_repository.expect_load_page_graph().returning(|| {
            let mut page_graph = PageGraph::default();
            page_graph.add_page({
                let page_id = PageId::from_str("20210203T040506Z")?;
                let page_content = PageContent::from(
                    vec![
                        "# title1",
                        "## Obsoletes",
                        "",
                        "- [20210203T040506Z](/pages/20210203T040506Z)",
                        "",
                    ]
                    .join("\n"),
                );
                Page::new(page_id, page_content)
            });
            page_graph.add_page({
                let page_id = PageId::from_str("20210203T040507Z")?;
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
                Page::new(page_id, page_content)
            });
            Ok(page_graph)
        });
        let app = TestApp { page_repository };
        let titles = app.list_titles_use_case().list_titles(false)?;
        assert_eq!(titles, vec![(PageTitle::from("title2".to_string()), false)]);
        Ok(())
    }

    #[test]
    fn test3() -> anyhow::Result<()> {
        let mut page_repository = MockPageRepository::new();
        page_repository.expect_load_page_graph().returning(|| {
            let mut page_graph = PageGraph::default();
            page_graph.add_page({
                let page_id = PageId::from_str("20210203T040506Z")?;
                let page_content = PageContent::from(
                    vec![
                        "# title1",
                        "## Obsoletes",
                        "",
                        "- [20210203T040506Z](/pages/20210203T040506Z)",
                        "",
                    ]
                    .join("\n"),
                );
                Page::new(page_id, page_content)
            });
            page_graph.add_page({
                let page_id = PageId::from_str("20210203T040507Z")?;
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
                Page::new(page_id, page_content)
            });
            Ok(page_graph)
        });
        let app = TestApp { page_repository };
        let titles = app.list_titles_use_case().list_titles(true)?;
        assert_eq!(
            titles,
            vec![
                (PageTitle::from("title2".to_string()), false),
                (PageTitle::from("title1".to_string()), true),
            ]
        );
        Ok(())
    }
}
