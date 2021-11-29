use std::convert::TryFrom;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("parse column number error")]
pub struct ParseColumnNumberError;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColumnNumber(usize);

impl std::convert::TryFrom<usize> for ColumnNumber {
    type Error = ParseColumnNumberError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(ParseColumnNumberError);
        }
        Ok(Self(value))
    }
}

impl From<ColumnNumber> for usize {
    fn from(column_number: ColumnNumber) -> Self {
        column_number.0
    }
}

impl std::fmt::Display for ColumnNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ColumnNumber {
    type Err = ParseColumnNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u = s.parse::<usize>().map_err(|_| ParseColumnNumberError)?;
        Self::try_from(u)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn str_conversion_test() -> anyhow::Result<()> {
        let column_number = ColumnNumber::from_str("1")?;
        assert_eq!(column_number.to_string(), "1");
        assert!(ColumnNumber::from_str("0").is_err());
        Ok(())
    }

    #[test]
    fn usize_conversion_test() -> anyhow::Result<()> {
        let column_number = ColumnNumber::try_from(1_usize)?;
        assert_eq!(usize::from(column_number), 1_usize);
        assert!(ColumnNumber::try_from(0_usize).is_err());
        Ok(())
    }
}
