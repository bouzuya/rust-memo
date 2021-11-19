use thiserror::Error;

use crate::PageId;

#[derive(Debug, Error)]
#[error("parse page path error")]
pub struct ParsePagePathError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PagePath(PageId);

impl std::str::FromStr for PagePath {
    type Err = ParsePagePathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s.strip_prefix("/pages/").ok_or(ParsePagePathError)?;
        PageId::from_str(stripped)
            .map(PagePath::from)
            .map_err(|_| ParsePagePathError)
    }
}

impl std::fmt::Display for PagePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/pages/{}", self.0)
    }
}

impl From<PageId> for PagePath {
    fn from(page_id: PageId) -> Self {
        Self(page_id)
    }
}

impl From<PagePath> for PageId {
    fn from(page_path: PagePath) -> Self {
        page_path.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn page_id_conversion_test() -> anyhow::Result<()> {
        let page_id = PageId::from_str("20210203T040506Z")?;
        assert_eq!(PageId::from(PagePath::from(page_id)), page_id);
        Ok(())
    }

    #[test]
    fn str_conversion_test() -> anyhow::Result<()> {
        let page_path = PagePath::from_str("/pages/20210203T040506Z")?;
        assert_eq!(page_path.to_string(), "/pages/20210203T040506Z");
        Ok(())
    }
}
