use crate::{error::ErrorKind, Error};

/// A database id.
///
/// # Limit
///
/// <https://firebase.google.com/docs/firestore/reference/rest/v1/projects.databases/create#query-parameters>
///
/// > This value should be 4-63 characters. Valid characters are /[a-z][0-9]-/ with first character a letter and the last a letter or a number. Must not be UUID-like /[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}/.
/// >
/// > "(default)" database id is also valid.
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use firestore_path::DatabaseId;
/// use std::str::FromStr;
///
/// let database_id = DatabaseId::from_str("my-database")?;
/// assert_eq!(database_id.as_ref(), "my-database");
/// assert_eq!(database_id.to_string(), "my-database");
///
/// let database_id = DatabaseId::from_str("(default)")?;
/// assert_eq!(database_id.as_ref(), "(default)");
/// assert_eq!(database_id.to_string(), "(default)");
///
/// let database_id = DatabaseId::default();
/// assert_eq!(database_id.as_ref(), "(default)");
/// assert_eq!(database_id.to_string(), "(default)");
/// #     Ok(())
/// # }
/// ```
///
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
            return Err(Error::from(ErrorKind::LengthOutOfBounds));
        }

        if !s
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(Error::from(ErrorKind::ContainsInvalidCharacter));
        }

        let first_char = s.chars().next().expect("already length checked");
        if !first_char.is_ascii_lowercase() {
            return Err(Error::from(ErrorKind::StartsWithNonLetter));
        }

        if s.ends_with('-') {
            return Err(Error::from(ErrorKind::EndsWithHyphen));
        }

        Ok(Self(s))
    }
}

impl std::default::Default for DatabaseId {
    /// Returns a new instance with the value `"(default)"`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use firestore_path::DatabaseId;
    /// assert_eq!(DatabaseId::default().to_string(), "(default)");
    /// ```
    fn default() -> Self {
        Self("(default)".to_string())
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
            ("", false),
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
