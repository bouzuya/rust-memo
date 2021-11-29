use std::convert::TryFrom;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("parse line number error")]
pub struct ParseLineNumberError;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LineNumber(usize);

impl std::convert::TryFrom<usize> for LineNumber {
    type Error = ParseLineNumberError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(ParseLineNumberError);
        }
        Ok(Self(value))
    }
}

impl From<LineNumber> for usize {
    fn from(line_number: LineNumber) -> Self {
        line_number.0
    }
}

impl std::fmt::Display for LineNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for LineNumber {
    type Err = ParseLineNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u = s.parse::<usize>().map_err(|_| ParseLineNumberError)?;
        Self::try_from(u)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn str_conversion_test() -> anyhow::Result<()> {
        let line_number = LineNumber::from_str("1")?;
        assert_eq!(line_number.to_string(), "1");
        assert!(LineNumber::from_str("0").is_err());
        Ok(())
    }

    #[test]
    fn usize_conversion_test() -> anyhow::Result<()> {
        let line_number = LineNumber::try_from(1_usize)?;
        assert_eq!(usize::from(line_number), 1_usize);
        assert!(LineNumber::try_from(0_usize).is_err());
        Ok(())
    }
}
