use anyhow::{anyhow, Context};
use entity::PageId;

use crate::{HasPageRepository, PageRepository};

pub trait EnsureLinksUseCase: HasPageRepository {
    fn ensure_links(&self, page_id: Option<&PageId>) -> anyhow::Result<()> {
        let f = |page_id: &PageId| -> anyhow::Result<()> {
            let mut page_content = self
                .page_repository()
                .find_content(page_id)?
                .with_context(|| anyhow!("file not found: {}", page_id))?;
            page_content.ensure_links();
            self.page_repository().save_content(page_id, page_content)?;
            Ok(())
        };
        match page_id {
            Some(page_id) => f(page_id),
            None => self
                .page_repository()
                .find_ids()?
                .into_iter()
                .try_for_each(|page_id| f(&page_id)),
        }
    }
}

impl<T: HasPageRepository> EnsureLinksUseCase for T {}

pub trait HasEnsureLinksUseCase {
    type EnsureLinksUseCase: EnsureLinksUseCase;

    fn ensure_links_use_case(&self) -> &Self::EnsureLinksUseCase;
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

    impl HasEnsureLinksUseCase for TestApp {
        type EnsureLinksUseCase = TestApp;

        fn ensure_links_use_case(&self) -> &Self::EnsureLinksUseCase {
            self
        }
    }

    #[test]
    fn none_test() -> anyhow::Result<()> {
        let mut page_repository = MockPageRepository::new();
        let page_id1 = PageId::from_str("20210203T040506Z")?;
        let page_id2 = PageId::from_str("20210203T040507Z")?;
        page_repository
            .expect_find_ids()
            .returning(move || Ok(vec![page_id1, page_id2]));
        page_repository
            .expect_find_content()
            .with(predicate::eq(page_id1))
            .returning(|_| {
                Ok(Some(PageContent::from(
                    vec!["# title", "", "[link1]", ""].join("\n"),
                )))
            });
        page_repository
            .expect_save_content()
            .with(
                predicate::eq(page_id1),
                predicate::eq(PageContent::from(
                    vec!["# title", "", "[link1]", "", "[link1]: /titles/link1", ""].join("\n"),
                )),
            )
            .returning(|_, _| Ok(()));
        page_repository
            .expect_find_content()
            .with(predicate::eq(page_id2))
            .returning(|_| {
                Ok(Some(PageContent::from(
                    vec!["# title", "", "[link2]", ""].join("\n"),
                )))
            });
        page_repository
            .expect_save_content()
            .with(
                predicate::eq(page_id2),
                predicate::eq(PageContent::from(
                    vec!["# title", "", "[link2]", "", "[link2]: /titles/link2", ""].join("\n"),
                )),
            )
            .returning(|_, _| Ok(()));
        let app = TestApp { page_repository };
        app.ensure_links_use_case().ensure_links(None)?;
        Ok(())
    }

    #[test]
    fn some_test() -> anyhow::Result<()> {
        let mut page_repository = MockPageRepository::new();
        let page_id = PageId::from_str("20210203T040506Z")?;
        page_repository
            .expect_find_content()
            .with(predicate::eq(page_id))
            .returning(|_| {
                Ok(Some(PageContent::from(
                    vec!["# title", "", "content1", "", "[link1]", ""].join("\n"),
                )))
            });
        page_repository
            .expect_save_content()
            .with(
                predicate::eq(page_id),
                predicate::eq(PageContent::from(
                    vec![
                        "# title",
                        "",
                        "content1",
                        "",
                        "[link1]",
                        "",
                        "[link1]: /titles/link1",
                        "",
                    ]
                    .join("\n"),
                )),
            )
            .returning(|_, _| Ok(()));
        let app = TestApp { page_repository };
        app.ensure_links_use_case().ensure_links(Some(&page_id))?;
        Ok(())
    }
}
