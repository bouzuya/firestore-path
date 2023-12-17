#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("todo")]
    ToDo,
}

/// limit: <https://firebase.google.com/docs/firestore/quotas#collections_documents_and_fields>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CollectionId(String);

impl std::convert::AsRef<str> for CollectionId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::convert::TryFrom<&str> for CollectionId {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_from(s.to_string())
    }
}

impl std::convert::TryFrom<String> for CollectionId {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        // <https://firebase.google.com/docs/firestore/quotas#collections_documents_and_fields>
        if s.len() > 1500 {
            return Err(Error::ToDo);
        }
        if s.contains('/') {
            return Err(Error::ToDo);
        }
        if s == "." || s == ".." {
            return Err(Error::ToDo);
        }
        if s.starts_with("__") && s.ends_with("__") {
            return Err(Error::ToDo);
        }
        Ok(Self(s))
    }
}

impl std::fmt::Display for CollectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for CollectionId {
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
        let s = "chatrooms";
        let collection_id = CollectionId::from_str(s)?;
        assert_eq!(collection_id.to_string(), s);

        let s = "messages";
        let collection_id = CollectionId::from_str(s)?;
        assert_eq!(collection_id.to_string(), s);

        assert_eq!(collection_id.as_ref(), s);
        Ok(())
    }

    #[test]
    fn test_impl_from_str_and_impl_try_from_string() -> anyhow::Result<()> {
        for (s, expected) in [
            ("x".repeat(1501).as_ref(), false),
            ("x".repeat(1500).as_ref(), true),
            ("chat/rooms", false),
            (".", false),
            (".x", true),
            ("..", false),
            ("..x", true),
            ("__x__", false),
            ("__x", true),
            ("x__", true),
        ] {
            assert_eq!(CollectionId::from_str(s).is_ok(), expected);
            assert_eq!(CollectionId::try_from(s).is_ok(), expected);
            assert_eq!(CollectionId::try_from(s.to_string()).is_ok(), expected);
            if expected {
                assert_eq!(CollectionId::from_str(s)?, CollectionId::try_from(s)?);
                assert_eq!(
                    CollectionId::from_str(s)?,
                    CollectionId::try_from(s.to_string())?
                );
                assert_eq!(CollectionId::from_str(s)?.to_string(), s);
            }
        }
        Ok(())
    }
}
