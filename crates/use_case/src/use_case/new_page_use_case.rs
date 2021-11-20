use anyhow::Context;
use entity::{PageContent, PageId, PageTitle};

use crate::{HasPageRepository, PageRepository};

pub trait NewPageUseCase: HasPageRepository {
    fn new_page(&self, title: PageTitle) -> anyhow::Result<PageId> {
        let page_id = PageId::new().context("This application is out of date.")?;
        let content = PageContent::from(format!("# {}", title));
        self.page_repository()
            .save_content(&page_id, content)
            .map(|_| page_id)
    }
}

impl<T: HasPageRepository> NewPageUseCase for T {}

pub trait HasNewPageUseCase {
    type NewPageUseCase: NewPageUseCase;

    fn new_page_use_case(&self) -> &Self::NewPageUseCase;
}

#[cfg(test)]
mod tests {
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
            .expect_save_content()
            // TODO: test new_page_id & content
            .returning(|_, _| Ok(()));
        let app = TestApp { page_repository };
        let page_title = PageTitle::from("title1".to_string());
        let _new_page_id = app.new_page_use_case().new_page(page_title)?;
        // TODO: test _new_page_id
        Ok(())
    }
}
