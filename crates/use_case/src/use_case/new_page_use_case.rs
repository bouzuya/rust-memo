use anyhow::Context;
use entity::{PageContent, PageId, PageTitle};

use crate::{HasPageRepository, PageRepository};

pub trait NewPageUseCase: HasPageRepository {
    fn new_page(&self, page_title: PageTitle) -> anyhow::Result<(PageId, bool)> {
        let page_graph = self.page_repository().load_page_graph()?;
        // TODO: already_exists is always true when page_title is empty
        let already_exists = !page_graph.titled(&page_title).is_empty();
        let page_id = PageId::new().context("This application is out of date.")?;
        let content = PageContent::from(format!("# {}", page_title));
        self.page_repository().save_content(&page_id, content)?;
        Ok((page_id, already_exists))
    }
}

impl<T: HasPageRepository> NewPageUseCase for T {}

pub trait HasNewPageUseCase {
    type NewPageUseCase: NewPageUseCase;

    fn new_page_use_case(&self) -> &Self::NewPageUseCase;
}

#[cfg(test)]
mod tests {
    use entity::PageGraph;

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

    impl HasNewPageUseCase for TestApp {
        type NewPageUseCase = TestApp;

        fn new_page_use_case(&self) -> &Self::NewPageUseCase {
            self
        }
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        let mut page_repository = MockPageRepository::new();
        page_repository
            .expect_load_page_graph()
            .returning(|| Ok(PageGraph::default()));
        page_repository
            .expect_save_content()
            // TODO: test new_page_id & content
            .returning(|_, _| Ok(()));
        let app = TestApp { page_repository };
        let page_title = PageTitle::from("title1".to_string());
        let (_new_page_id, already_exists) = app.new_page_use_case().new_page(page_title)?;
        assert!(!already_exists);
        // TODO: test _new_page_id
        Ok(())
    }
}
