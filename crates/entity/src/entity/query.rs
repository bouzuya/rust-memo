use thiserror::Error;

#[derive(Debug, Error)]
#[error("parse query error")]
pub struct ParseQueryError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query(String);

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
    fn str_conversion_test() -> anyhow::Result<()> {
        let query = Query::from_str("query1")?;
        assert_eq!(query.to_string(), "query1");
        Ok(())
    }
}
