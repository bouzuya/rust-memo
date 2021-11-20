use thiserror::Error;

use crate::{PageId, PageTitle};

#[derive(Debug, Error)]
#[error("parse page path error")]
pub struct ParsePagePathError;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PageLinkTo {
    // Page(PageId),
    Title(PageTitle),
    // TODO: Others
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PageLink {
    from: PageId,
    // TODO: label: String,
    to: PageLinkTo,
}

impl PageLink {
    pub fn new(from: PageId, to: PageLinkTo) -> Self {
        Self { from, to }
    }

    pub fn from(&self) -> &PageId {
        &self.from
    }

    pub fn to(&self) -> &PageLinkTo {
        &self.to
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let page_id = PageId::from_str("20210203T040506Z")?;
        let page_title = PageTitle::from("title1".to_string());
        let page_link_to = PageLinkTo::Title(page_title);
        let page_link = PageLink::new(page_id, page_link_to.clone());
        assert_eq!(page_link.from(), &page_id);
        assert_eq!(page_link.to(), &page_link_to);
        Ok(())
    }
}
