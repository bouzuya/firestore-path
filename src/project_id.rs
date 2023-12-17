#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("todo")]
    ToDo,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProjectId(String);

impl std::convert::AsRef<str> for ProjectId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::convert::TryFrom<&str> for ProjectId {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_from(s.to_string())
    }
}

impl std::convert::TryFrom<String> for ProjectId {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        // <https://cloud.google.com/resource-manager/docs/creating-managing-projects>

        if !(6..=30).contains(&s.len()) {
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
        if last_char == '-' {
            return Err(Error::ToDo);
        }

        if s.contains("google")
            || s.contains("null")
            || s.contains("undefined")
            || s.contains("ssl")
        {
            return Err(Error::ToDo);
        }

        Ok(Self(s))
    }
}

impl std::fmt::Display for ProjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for ProjectId {
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
        let s = "my-project";
        let project_id = ProjectId::from_str(s)?;
        assert_eq!(project_id.to_string(), s);

        assert_eq!(project_id.as_ref(), s);
        Ok(())
    }

    #[test]
    fn test_impl_from_str_and_impl_try_from_string() -> anyhow::Result<()> {
        for (s, expected) in [
            ("x".repeat(5).as_ref(), false),
            ("x".repeat(6).as_ref(), true),
            ("x".repeat(30).as_ref(), true),
            ("x".repeat(31).as_ref(), false),
            ("chat/rooms", false),
            ("xxxxxx", true),
            ("x-xxxx", true),
            ("x0xxxx", true),
            ("xAxxxx", false),
            ("0xxxxx", false),
            ("xxxxx0", true),
            ("xxxxx-", false),
            ("xgoogle", false),
            ("xnull", false),
            ("xundefined", false),
            ("xssl", false),
        ] {
            assert_eq!(ProjectId::from_str(s).is_ok(), expected);
            assert_eq!(ProjectId::try_from(s).is_ok(), expected);
            assert_eq!(ProjectId::try_from(s.to_string()).is_ok(), expected);
            if expected {
                assert_eq!(ProjectId::from_str(s)?, ProjectId::try_from(s)?);
                assert_eq!(ProjectId::from_str(s)?, ProjectId::try_from(s.to_string())?);
                assert_eq!(ProjectId::from_str(s)?.to_string(), s);
            }
        }
        Ok(())
    }
}
