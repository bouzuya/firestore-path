#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("todo")]
    ToDo,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DatabaseId(String);

impl std::convert::AsRef<str> for DatabaseId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::convert::TryFrom<&str> for DatabaseId {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_from(s.to_string())
    }
}

impl std::convert::TryFrom<String> for DatabaseId {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        // <https://firebase.google.com/docs/firestore/reference/rest/v1/projects.databases/create#query-parameters>
        if s == "(default)" {
            return Ok(Self(s.to_string()));
        }

        if !(4..=63).contains(&s.len()) {
            return Err(Error::ToDo);
        }

        if !s
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(Error::ToDo);
        }

        let first_char = s.chars().next().expect("already length checked");
        if !first_char.is_ascii_lowercase() {
            return Err(Error::ToDo);
        }

        let last_char = s.chars().next_back().expect("already length checked");
        if !(last_char.is_ascii_lowercase() || last_char.is_ascii_digit()) {
            return Err(Error::ToDo);
        }

        Ok(Self(s))
    }
}

impl std::fmt::Display for DatabaseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for DatabaseId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "my-database";
        let database_id = DatabaseId::from_str(s)?;
        assert_eq!(database_id.to_string(), s);

        let s = "(default)";
        let database_id = DatabaseId::from_str(s)?;
        assert_eq!(database_id.to_string(), s);

        assert_eq!(database_id.as_ref(), s);
        Ok(())
    }

    #[test]
    fn test_impl_from_str() -> anyhow::Result<()> {
        for (s, expected) in [
            ("(default)", true),
            ("(default1)", false),
            ("x".repeat(3).as_str(), false),
            ("x".repeat(4).as_str(), true),
            ("x".repeat(63).as_str(), true),
            ("x".repeat(64).as_str(), false),
            ("x1-x", true),
            ("xAxx", false),
            ("-xxx", false),
            ("0xxx", false),
            ("xxx-", false),
            ("xxx0", true),
        ] {
            assert_eq!(DatabaseId::from_str(s).is_ok(), expected);
            assert_eq!(DatabaseId::try_from(s).is_ok(), expected);
            assert_eq!(DatabaseId::try_from(s.to_string()).is_ok(), expected);
            if expected {
                assert_eq!(DatabaseId::from_str(s)?, DatabaseId::try_from(s)?);
                assert_eq!(
                    DatabaseId::from_str(s)?,
                    DatabaseId::try_from(s.to_string())?
                );
                assert_eq!(DatabaseId::from_str(s)?.to_string(), s);
            }
        }
        Ok(())
    }
}
