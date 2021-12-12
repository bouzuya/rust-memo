use std::collections::BTreeSet;

use anyhow::{anyhow, Context};
use entity::{PageId, PageIdOrPageTitle};

use crate::{HasPageRepository, PageRepository};

pub trait EditPageUseCase: HasPageRepository {
    fn edit_page(&self, page_id: &PageIdOrPageTitle) -> anyhow::Result<(PageId, PageId, bool)> {
        let page_graph = self.page_repository().load_page_graph()?;
        let page_id = match page_id {
            PageIdOrPageTitle::PageId(page_id) => *page_id,
            PageIdOrPageTitle::PageTitle(page_title) => {
                let page_ids = page_graph.titled(page_title);
                let filtered_page_ids = page_ids
                    .into_iter()
                    .filter(|page_id| !page_graph.is_obsoleted(page_id))
                    .collect::<BTreeSet<_>>();
                filtered_page_ids
                    .into_iter()
                    .rev()
                    .next()
                    .ok_or_else(|| anyhow!("title not found"))?
            }
        };
        let page = self
            .page_repository()
            .find_by_id(&page_id)?
            .with_context(|| anyhow!("file not found: {}", page_id))?;
        let is_obsoleted = page_graph.is_obsoleted(&page_id);
        let mut page_content = page.content().clone();
        page_content.replace_obsoletes(page_id);
        let new_page_id = PageId::new().context("This application is out of date.")?;
        self.page_repository()
            .save_content(&new_page_id, page_content)?;
        Ok((page_id, new_page_id, is_obsoleted))
    }
}

impl<T: HasPageRepository> EditPageUseCase for T {}

pub trait HasEditPageUseCase {
    type EditPageUseCase: EditPageUseCase;

    fn edit_page_use_case(&self) -> &Self::EditPageUseCase;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use entity::{Page, PageContent, PageGraph};
    use mockall::predicate;

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

    impl HasEditPageUseCase for TestApp {
        type EditPageUseCase = TestApp;

        fn edit_page_use_case(&self) -> &Self::EditPageUseCase {
            self
        }
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        let mut page_repository = MockPageRepository::new();
        let page_id = PageId::from_str("20210203T040506Z")?;
        page_repository
            .expect_load_page_graph()
            .returning(|| Ok(PageGraph::default()));
        page_repository
            .expect_find_by_id()
            .with(predicate::eq(page_id))
            .returning(move |_| {
                Ok(Some(Page::new(
                    page_id,
                    PageContent::from("# title\n\ncontent1".to_string()),
                )))
            });
        page_repository
            .expect_save_content()
            // TODO: test new_page_id & content
            .returning(|_, _| Ok(()));
        let app = TestApp { page_repository };
        let _new_page_id = app
            .edit_page_use_case()
            .edit_page(&PageIdOrPageTitle::PageId(page_id))?;
        // TODO: test _new_page_id
        Ok(())
    }
}
