use thiserror::Error;

use crate::PageTitle;

#[derive(Debug, Error)]
#[error("parse title path error")]
pub struct ParseTitlePathError;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TitlePath(PageTitle);

impl std::str::FromStr for TitlePath {
    type Err = ParseTitlePathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s
            .strip_prefix("/titles/")
            .and_then(|s| percent_encoding::percent_decode_str(s).decode_utf8().ok())
            .ok_or(ParseTitlePathError)?;
        Ok(TitlePath::from(PageTitle::from(stripped.to_string())))
    }
}

impl std::fmt::Display for TitlePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "/titles/{}",
            percent_encoding::utf8_percent_encode(
                self.0.as_str(),
                percent_encoding::NON_ALPHANUMERIC
            ),
        )
    }
}

impl From<PageTitle> for TitlePath {
    fn from(page_title: PageTitle) -> Self {
        Self(page_title)
    }
}

impl From<TitlePath> for PageTitle {
    fn from(page_path: TitlePath) -> Self {
        page_path.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn page_title_conversion_test() {
        let page_title = PageTitle::from("title1".to_string());
        assert_eq!(
            PageTitle::from(TitlePath::from(page_title.clone())),
            page_title
        );
    }

    #[test]
    fn str_conversion_test() -> anyhow::Result<()> {
        let page_path = TitlePath::from_str("/titles/title1")?;
        assert_eq!(page_path.to_string(), "/titles/title1");
        let page_path = TitlePath::from_str("/titles/%E3%81%82")?;
        assert_eq!(page_path.to_string(), "/titles/%E3%81%82");
        assert_eq!(
            PageTitle::from(page_path),
            PageTitle::from("あ".to_string())
        );
        Ok(())
    }
}
