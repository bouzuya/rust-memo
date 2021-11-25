use thiserror::Error;

#[derive(Debug, Error)]
#[error("parse query error")]
pub struct ParseQueryError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query(String);

impl Query {
    pub fn matches(&self, content: &str) -> Vec<(usize, usize)> {
        let mut matches = vec![];
        for (line, line_content) in content.lines().enumerate() {
            if let Some(col) = line_content.find(self.0.as_str()) {
                matches.push((line + 1, col + 1));
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
        let query = Query::from_str("a")?;
        assert!(query.matches("").is_empty());
        assert_eq!(query.matches("a"), vec![(1, 1)]);
        assert_eq!(query.matches("ba"), vec![(1, 2)]);
        assert_eq!(query.matches("aba"), vec![(1, 1)]);
        assert_eq!(query.matches("\n"), vec![]);
        assert_eq!(query.matches("a\n"), vec![(1, 1)]);
        assert_eq!(query.matches("a\na"), vec![(1, 1), (2, 1)]);
        assert_eq!(query.matches("a\nba"), vec![(1, 1), (2, 2)]);
        assert_eq!(query.matches("\na"), vec![(2, 1)]);
        Ok(())
    }

    #[test]
    fn str_conversion_test() -> anyhow::Result<()> {
        let query = Query::from_str("query1")?;
        assert_eq!(query.to_string(), "query1");
        Ok(())
    }
}
