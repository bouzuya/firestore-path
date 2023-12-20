use std::str::FromStr;

use crate::{error::ErrorKind, CollectionId, DocumentId, DocumentPath, Error};

/// A collection path.
///
/// # Format
///
/// - `{collection_id}`
/// - `{document_path}/{collection_id}`
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use firestore_path::CollectionPath;
/// use std::str::FromStr;
///
/// let collection_path = CollectionPath::from_str("chatrooms")?;
/// assert_eq!(collection_path.to_string(), "chatrooms");
///
/// let collection_path = CollectionPath::from_str("chatrooms/chatroom1/messages")?;
/// assert_eq!(collection_path.to_string(), "chatrooms/chatroom1/messages");
/// #     Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CollectionPath {
    document_path: Option<DocumentPath>,
    collection_id: CollectionId,
}

impl CollectionPath {
    /// Create a new `CollectionPath`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,CollectionPath,DocumentPath};
    /// use std::str::FromStr;
    ///
    /// let collection_id = CollectionId::from_str("chatrooms")?;
    /// let collection_path = CollectionPath::new(None, collection_id);
    /// assert_eq!(collection_path.to_string(), "chatrooms");
    ///
    /// let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
    /// let collection_id = CollectionId::from_str("messages")?;
    /// let collection_path = CollectionPath::new(Some(document_path), collection_id);
    /// assert_eq!(collection_path.to_string(), "chatrooms/chatroom1/messages");
    /// #     Ok(())
    /// # }
    /// ```
    pub fn new(parent: Option<DocumentPath>, collection_id: CollectionId) -> Self {
        Self {
            document_path: parent,
            collection_id,
        }
    }

    /// Returns the `CollectionId` of this `CollectionPath`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,CollectionPath,DocumentPath};
    /// use std::str::FromStr;
    ///
    /// let collection_path = CollectionPath::from_str("chatrooms")?;
    /// assert_eq!(collection_path.collection_id(), &CollectionId::from_str("chatrooms")?);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn collection_id(&self) -> &CollectionId {
        &self.collection_id
    }

    /// Create a new `DocumentPath` from this `CollectionPath` and `document_id`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionPath,DocumentPath};
    /// use std::str::FromStr;
    ///
    /// let collection_path = CollectionPath::from_str("chatrooms")?;
    /// assert_eq!(collection_path.doc("chatroom1")?, DocumentPath::from_str("chatrooms/chatroom1")?);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn doc<E, T>(self, document_id: T) -> Result<DocumentPath, Error>
    where
        E: std::fmt::Display,
        T: TryInto<DocumentId, Error = E>,
    {
        let document_id = document_id
            .try_into()
            .map_err(|e| Error::from(ErrorKind::DocumentIdConversion(e.to_string())))?;
        let document_path = DocumentPath::new(self, document_id);
        Ok(document_path)
    }

    /// Returns the parent `DocumentPath` of this `CollectionPath`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionPath,DocumentPath};
    /// use std::str::FromStr;
    ///
    /// let collection_path = CollectionPath::from_str("chatrooms")?;
    /// assert_eq!(collection_path.parent(), None);
    ///
    /// let collection_path = CollectionPath::from_str("chatrooms/chatroom1/messages")?;
    /// assert_eq!(collection_path.parent(), Some(&DocumentPath::from_str("chatrooms/chatroom1")?));
    /// #     Ok(())
    /// # }
    /// ```
    pub fn parent(&self) -> Option<&DocumentPath> {
        self.document_path.as_ref()
    }

    pub(crate) fn into_tuple(self) -> (Option<DocumentPath>, CollectionId) {
        (self.document_path, self.collection_id)
    }
}

impl std::convert::From<CollectionId> for CollectionPath {
    fn from(collection_id: CollectionId) -> Self {
        CollectionPath::new(None, collection_id)
    }
}

impl std::convert::From<CollectionPath> for CollectionId {
    fn from(collection_path: CollectionPath) -> Self {
        collection_path.collection_id
    }
}

impl std::convert::From<CollectionPath> for Option<DocumentPath> {
    fn from(collection_path: CollectionPath) -> Self {
        collection_path.document_path
    }
}

impl std::convert::TryFrom<&str> for CollectionPath {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_from(s.to_string())
    }
}

impl std::convert::TryFrom<String> for CollectionPath {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(match s.rsplit_once('/') {
            Some((document_path, collection_id)) => Self {
                document_path: Some(DocumentPath::from_str(document_path)?),
                collection_id: CollectionId::from_str(collection_id)?,
            },
            None => Self {
                document_path: None,
                collection_id: CollectionId::try_from(s)?,
            },
        })
    }
}

impl std::fmt::Display for CollectionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.document_path.as_ref() {
            Some(document_path) => write!(f, "{}/{}", document_path, self.collection_id),
            None => self.collection_id.fmt(f),
        }
    }
}

impl std::str::FromStr for CollectionPath {
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
        let collection_path = CollectionPath::from_str(s)?;
        assert_eq!(collection_path.to_string(), s);

        let s = "chatrooms/chatroom1/messages";
        let collection_path = CollectionPath::from_str(s)?;
        assert_eq!(collection_path.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_collection_id() -> anyhow::Result<()> {
        let collection_path = CollectionPath::from_str("chatrooms")?;
        assert_eq!(
            collection_path.collection_id(),
            &CollectionId::from_str("chatrooms")?
        );
        Ok(())
    }

    #[test]
    fn test_doc() -> anyhow::Result<()> {
        let collection_path = CollectionPath::from_str("chatrooms")?;
        let document_path = collection_path.doc("chatroom1")?;
        assert_eq!(
            document_path,
            DocumentPath::from_str("chatrooms/chatroom1")?
        );

        let collection_path = CollectionPath::from_str("chatrooms/chatroom1/messages")?;
        let document_path = collection_path.doc("message1")?;
        assert_eq!(
            document_path,
            DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?
        );

        let collection_path = CollectionPath::from_str("chatrooms")?;
        let document_id = DocumentId::from_str("chatroom1")?;
        let document_path = collection_path.doc(document_id)?;
        assert_eq!(
            document_path,
            DocumentPath::from_str("chatrooms/chatroom1")?
        );

        Ok(())
    }

    #[test]
    fn test_impl_from_collection_path_for_document_path() -> anyhow::Result<()> {
        let collection_path = CollectionPath::from_str("chatrooms")?;
        assert_eq!(Option::<DocumentPath>::from(collection_path), None);

        let collection_path = CollectionPath::from_str("chatrooms/chatroom1/messages")?;
        assert_eq!(
            Option::<DocumentPath>::from(collection_path),
            Some(DocumentPath::from_str("chatrooms/chatroom1")?)
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_collection_id_for_collection_path() -> anyhow::Result<()> {
        let collection_id = CollectionId::from_str("chatrooms")?;
        assert_eq!(
            CollectionPath::from(collection_id),
            CollectionPath::from_str("chatrooms")?
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_collection_path_for_collection_id() -> anyhow::Result<()> {
        let collection_path = CollectionPath::from_str("chatrooms")?;
        assert_eq!(
            CollectionId::from(collection_path),
            CollectionId::from_str("chatrooms")?
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_str_and_impl_try_from_string() -> anyhow::Result<()> {
        for (s, expected) in [("chatrooms", true), ("chatrooms/chatroom1/messages", true)] {
            assert_eq!(CollectionPath::from_str(s).is_ok(), expected);
            assert_eq!(CollectionPath::try_from(s).is_ok(), expected);
            assert_eq!(CollectionPath::try_from(s.to_string()).is_ok(), expected);
            if expected {
                assert_eq!(CollectionPath::from_str(s)?, CollectionPath::try_from(s)?);
                assert_eq!(
                    CollectionPath::from_str(s)?,
                    CollectionPath::try_from(s.to_string())?
                );
                assert_eq!(CollectionPath::from_str(s)?.to_string(), s);
            }
        }
        Ok(())
    }

    #[test]
    fn test_new() -> anyhow::Result<()> {
        let collection_id = build_collection_id()?;
        let collection_path = CollectionPath::new(None, collection_id.clone());
        assert_eq!(collection_path.to_string(), format!("{}", collection_id));

        let document_path = build_document_path()?;
        let collection_path =
            CollectionPath::new(Some(document_path.clone()), collection_id.clone());
        assert_eq!(
            collection_path.to_string(),
            format!("{}/{}", document_path, collection_id)
        );
        Ok(())
    }

    #[test]
    fn test_parent() -> anyhow::Result<()> {
        let collection_path = CollectionPath::from_str("chatrooms")?;
        assert_eq!(collection_path.parent(), None);

        let collection_path = CollectionPath::from_str("chatrooms/chatroom1/messages")?;
        assert_eq!(
            collection_path.parent(),
            Some(&DocumentPath::from_str("chatrooms/chatroom1")?)
        );
        Ok(())
    }

    fn build_collection_id() -> anyhow::Result<CollectionId> {
        Ok(CollectionId::from_str("chatrooms")?)
    }

    fn build_document_path() -> anyhow::Result<DocumentPath> {
        Ok(DocumentPath::from_str("chatrooms/chatroom1")?)
    }
}
