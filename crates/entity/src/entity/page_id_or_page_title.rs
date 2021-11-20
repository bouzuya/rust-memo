use thiserror::Error;

use crate::{PageId, PageTitle};

#[derive(Debug, Error)]
#[error("parse page id or page title error")]
pub struct ParsePageIdOrPageTitleError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PageIdOrPageTitle {
    PageId(PageId),
    PageTitle(PageTitle),
}

impl From<&str> for PageIdOrPageTitle {
    fn from(s: &str) -> Self {
        match PageId::from_like_str(s) {
            Ok(page_id) => Self::PageId(page_id),
            Err(_) => Self::PageTitle(PageTitle::from(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        assert_eq!(
            PageIdOrPageTitle::from("20210203T040506Z"),
            PageIdOrPageTitle::PageId(PageId::from_str("20210203T040506Z")?)
        );
        assert_eq!(
            PageIdOrPageTitle::from("http://localhost:8080/pages/20210203T040506Z"),
            PageIdOrPageTitle::PageId(PageId::from_str("20210203T040506Z")?)
        );
        assert_eq!(
            PageIdOrPageTitle::from("20210203T040506Z.md"),
            PageIdOrPageTitle::PageId(PageId::from_str("20210203T040506Z")?)
        );
        assert_eq!(
            PageIdOrPageTitle::from("title1"),
            PageIdOrPageTitle::PageTitle(PageTitle::from("title1".to_string()))
        );
        assert_eq!(
            PageIdOrPageTitle::from("あ"),
            PageIdOrPageTitle::PageTitle(PageTitle::from("あ".to_string()))
        );
        Ok(())
    }
}
