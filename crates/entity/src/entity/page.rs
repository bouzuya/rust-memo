use crate::{PageContent, PageId, PageTitle};

#[derive(Debug, Eq, PartialEq)]
pub struct Page {
    id: PageId,
    title: PageTitle,
    content: PageContent,
}

impl Page {
    pub fn new(id: PageId, title: PageTitle, content: PageContent) -> Self {
        Self { id, title, content }
    }

    pub fn id(&self) -> &PageId {
        &self.id
    }

    pub fn title(&self) -> &PageTitle {
        &self.title
    }

    pub fn content(&self) -> &PageContent {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{PageContent, PageId, PageTitle};

    use super::*;

    #[test]
    fn new_test() -> anyhow::Result<()> {
        let id = PageId::from_str("20210203T040506Z")?;
        let title = PageTitle::from_str("title1")?;
        let content = PageContent::from("# title1\n\ncontent1".to_string());
        let page = Page::new(id, title.clone(), content.clone());
        assert_eq!(page.id(), &id);
        assert_eq!(page.title(), &title);
        assert_eq!(page.content(), &content);
        Ok(())
    }
}
