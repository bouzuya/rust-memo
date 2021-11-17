use anyhow::{anyhow, Context};
use entity::PageId;

use crate::{HasPageRepository, PageRepository};

// TODO:
fn pages_url() -> String {
    "/pages".to_string()
}

// TODO:
fn page_url(page_id: &PageId) -> String {
    format!(
        "{}/{}",
        pages_url(),
        percent_encoding::utf8_percent_encode(
            &page_id.to_string(),
            percent_encoding::NON_ALPHANUMERIC,
        )
    )
}

pub trait EditUseCase: HasPageRepository {
    fn edit(&self, page_id: &PageId) -> anyhow::Result<PageId> {
        let mut content = self
            .page_repository()
            .find_content(page_id)?
            .with_context(|| anyhow!("file not found: {}", page_id))?;
        if let Some(index) = content.find("\n## Obsoletes") {
            content.truncate(index);
        }
        content.push_str(&format!(
            "\n## Obsoletes\n\n- [{}]({})",
            page_id.to_string(),
            page_url(page_id)
        ));
        let new_page_id = PageId::new().context("This application is out of date.")?;
        self.page_repository().save_content(&new_page_id, content)?;
        Ok(new_page_id)
    }
}

impl<T: HasPageRepository> EditUseCase for T {}

pub trait HasEditUseCase {
    type EditUseCase: EditUseCase;

    fn edit_use_case(&self) -> &Self::EditUseCase;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

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

    impl HasEditUseCase for TestApp {
        type EditUseCase = TestApp;

        fn edit_use_case(&self) -> &Self::EditUseCase {
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
            .returning(|_| Ok(Some("# title\n\ncontent1".to_string())));
        page_repository
            .expect_save_content()
            // TODO: test new_page_id & content
            .returning(|_, _| Ok(()));
        let app = TestApp { page_repository };
        let _new_page_id = app.edit_use_case().edit(&page_id)?;
        // TODO: test _new_page_id
        Ok(())
    }
}