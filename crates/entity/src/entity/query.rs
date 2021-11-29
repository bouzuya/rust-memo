use std::convert::TryFrom;

use thiserror::Error;

use crate::LineNumber;

#[derive(Debug, Error)]
#[error("parse query error")]
pub struct ParseQueryError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query(String);

impl Query {
    pub fn matches(&self, content: &str) -> Vec<(LineNumber, usize)> {
        let mut matches = vec![];
        for (line, line_content) in content.lines().enumerate() {
            if let Some(col) = line_content.find(self.0.as_str()) {
                matches.push((
                    LineNumber::try_from(line + 1).expect("invalid line"),
                    line_content[..col].chars().count() + 1,
                ));
            }
        }
        matches
    }
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Query {
    type Err = ParseQueryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn matches_test() -> anyhow::Result<()> {
        let l1 = LineNumber::try_from(1_usize)?;
        let l2 = LineNumber::try_from(1_usize)?;
        let query = Query::from_str("a")?;
        assert!(query.matches("").is_empty());
        assert_eq!(query.matches("a"), vec![(l1, 1)]);
        assert_eq!(query.matches("ba"), vec![(l1, 2)]);
        assert_eq!(query.matches("aba"), vec![(l1, 1)]);
        assert_eq!(query.matches("\n"), vec![]);
        assert_eq!(query.matches("a\n"), vec![(l1, 1)]);
        assert_eq!(query.matches("a\na"), vec![(l1, 1), (l2, 1)]);
        assert_eq!(query.matches("a\nba"), vec![(l1, 1), (l2, 2)]);
        assert_eq!(query.matches("\na"), vec![(l2, 1)]);

        let query = Query::from_str("あ")?;
        assert_eq!(query.matches("あいうえお"), vec![(l1, 1)]);
        let query = Query::from_str("く")?;
        assert_eq!(query.matches("あいうえお\nかきくけこ"), vec![(l2, 3)]);
        Ok(())
    }

    #[test]
    fn str_conversion_test() -> anyhow::Result<()> {
        let query = Query::from_str("query1")?;
        assert_eq!(query.to_string(), "query1");
        Ok(())
    }
}
