use anyhow::{anyhow, Context};
use entity::PageId;

use crate::{HasPageRepository, PageRepository};

pub trait EditPageUseCase: HasPageRepository {
    fn edit_page(&self, page_id: &PageId) -> anyhow::Result<PageId> {
        let mut page_content = self
            .page_repository()
            .find_content(page_id)?
            .with_context(|| anyhow!("file not found: {}", page_id))?;
        page_content.replace_obsoletes(*page_id);
        let new_page_id = PageId::new().context("This application is out of date.")?;
        // TODO: PageRepository::save_content(&self, page_id: &PageId, page_content: PageContent) -> anyhow::Result<()>
        self.page_repository()
            .save_content(&new_page_id, page_content.to_string())?;
        Ok(new_page_id)
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

    use entity::PageContent;
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
            .expect_find_content()
            .with(predicate::eq(page_id))
            .returning(|_| Ok(Some(PageContent::from("# title\n\ncontent1".to_string()))));
        page_repository
            .expect_save_content()
            // TODO: test new_page_id & content
            .returning(|_, _| Ok(()));
        let app = TestApp { page_repository };
        let _new_page_id = app.edit_page_use_case().edit_page(&page_id)?;
        // TODO: test _new_page_id
        Ok(())
    }
}
