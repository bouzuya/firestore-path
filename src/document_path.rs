use std::str::FromStr;

use crate::{error::ErrorKind, CollectionId, CollectionPath, DocumentId, Error};

/// A document path.
///
/// # Format
///
/// `{collection_path}/{document_id}`
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use firestore_path::DocumentPath;
/// use std::str::FromStr;
///
/// let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
/// assert_eq!(document_path.to_string(), "chatrooms/chatroom1");
/// #     Ok(())
/// # }
/// ```
///
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DocumentPath {
    collection_path: Box<CollectionPath>,
    document_id: DocumentId,
}

impl DocumentPath {
    /// Creates a new `DocumentPath`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionPath,DocumentId,DocumentPath};
    /// use std::str::FromStr;
    ///
    /// let collection_path = CollectionPath::from_str("chatrooms")?;
    /// let document_id = DocumentId::from_str("chatroom1")?;
    /// let document_path = DocumentPath::new(collection_path, document_id);
    /// assert_eq!(document_path.to_string(), "chatrooms/chatroom1");
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn new(collection_path: CollectionPath, document_id: DocumentId) -> Self {
        Self {
            collection_path: Box::new(collection_path),
            document_id,
        }
    }

    /// Creates a new `DocumentPath` from this `DocumentPath` and `collection_id`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionPath,DocumentPath};
    /// use std::str::FromStr;
    ///
    /// let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
    /// assert_eq!(
    ///     document_path.collection("messages")?,
    ///     CollectionPath::from_str("chatrooms/chatroom1/messages")?
    /// );
    ///
    /// let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
    /// assert_eq!(
    ///     document_path.collection("messages/message1/col")?,
    ///     CollectionPath::from_str("chatrooms/chatroom1/messages/message1/col")?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn collection<E, T>(self, collection_path: T) -> Result<CollectionPath, Error>
    where
        E: std::fmt::Display,
        T: TryInto<CollectionPath, Error = E>,
    {
        let mut collection_path: CollectionPath = collection_path
            .try_into()
            .map_err(|e| Error::from(ErrorKind::CollectionIdConversion(e.to_string())))?;

        enum I {
            C(CollectionId),
            D(DocumentId),
        }
        let mut path_components = vec![];
        loop {
            let (parent, collection_id) = collection_path.into_tuple();
            path_components.push(I::C(collection_id));
            if let Some(document_path) = parent {
                let (next_collection_path, document_id) = document_path.into_tuple();
                path_components.push(I::D(document_id));
                collection_path = next_collection_path;
            } else {
                break;
            }
        }

        enum P {
            C(CollectionPath),
            D(DocumentPath),
        }
        let mut result = P::D(self);
        for path_component in path_components.into_iter().rev() {
            result = match (result, path_component) {
                (P::C(_), I::C(_)) | (P::D(_), I::D(_)) => unreachable!(),
                (P::C(c), I::D(d)) => P::D(DocumentPath::new(c, d)),
                (P::D(d), I::C(c)) => P::C(CollectionPath::new(Some(d), c)),
            };
        }
        let collection_path = match result {
            P::C(c) => c,
            P::D(_) => unreachable!(),
        };
        Ok(collection_path)
    }

    /// Returns the `CollectionId` of this `DocumentPath`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,DocumentPath};
    /// use std::str::FromStr;
    ///
    /// let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
    /// assert_eq!(
    ///     document_path.collection_id(),
    ///     &CollectionId::from_str("chatrooms")?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn collection_id(&self) -> &CollectionId {
        self.collection_path.collection_id()
    }

    /// Returns the `DocumentId` of this `DocumentPath`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DocumentId,DocumentPath};
    /// use std::str::FromStr;
    ///
    /// let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
    /// assert_eq!(
    ///     document_path.document_id(),
    ///     &DocumentId::from_str("chatroom1")?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn document_id(&self) -> &DocumentId {
        &self.document_id
    }

    /// Returns the parent `CollectionPath` of this `DocumentPath`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionPath,DocumentPath};
    /// use std::str::FromStr;
    ///
    /// let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
    /// assert_eq!(
    ///     document_path.parent(),
    ///     &CollectionPath::from_str("chatrooms")?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn parent(&self) -> &CollectionPath {
        self.collection_path.as_ref()
    }

    pub(crate) fn into_tuple(self) -> (CollectionPath, DocumentId) {
        (*self.collection_path, self.document_id)
    }
}

impl std::convert::From<DocumentPath> for CollectionPath {
    fn from(document_path: DocumentPath) -> Self {
        *document_path.collection_path
    }
}

impl std::convert::From<DocumentPath> for DocumentId {
    fn from(document_path: DocumentPath) -> Self {
        document_path.document_id
    }
}

impl std::convert::TryFrom<&str> for DocumentPath {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s.rsplit_once('/') {
            Some((collection_path, document_id)) => Self {
                collection_path: Box::new(CollectionPath::from_str(collection_path)?),
                document_id: DocumentId::from_str(document_id)?,
            },
            None => {
                return Err(Error::from(ErrorKind::NotContainsSlash));
            }
        })
    }
}

impl std::convert::TryFrom<String> for DocumentPath {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(s.as_str())
    }
}

impl std::fmt::Display for DocumentPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.collection_path, self.document_id)
    }
}

impl std::str::FromStr for DocumentPath {
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
        let s = "chatrooms/chatroom1";
        let document_path = DocumentPath::from_str(s)?;
        assert_eq!(document_path.to_string(), s);

        let s = "chatrooms/chatroom1/messages/message1";
        let document_path = DocumentPath::from_str(s)?;
        assert_eq!(document_path.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_collection() -> anyhow::Result<()> {
        let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
        let collection_path = document_path.collection("messages")?;
        assert_eq!(
            collection_path,
            CollectionPath::from_str("chatrooms/chatroom1/messages")?
        );

        let document_path = DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?;
        let collection_path = document_path.collection("col")?;
        assert_eq!(
            collection_path,
            CollectionPath::from_str("chatrooms/chatroom1/messages/message1/col")?
        );

        let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
        let collection_id = CollectionId::from_str("messages")?;
        let collection_path = document_path.collection(collection_id)?;
        assert_eq!(
            collection_path,
            CollectionPath::from_str("chatrooms/chatroom1/messages")?
        );
        Ok(())
    }

    #[test]
    fn test_collection_with_colleciton_path() -> anyhow::Result<()> {
        let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
        let collection_path = document_path.collection("messages/message1/col")?;
        assert_eq!(
            collection_path,
            CollectionPath::from_str("chatrooms/chatroom1/messages/message1/col")?
        );

        let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
        let collection_path = CollectionPath::from_str("messages/message1/col")?;
        let collection_path = document_path.collection(collection_path)?;
        assert_eq!(
            collection_path,
            CollectionPath::from_str("chatrooms/chatroom1/messages/message1/col")?
        );
        Ok(())
    }

    #[test]
    fn test_document_id() -> anyhow::Result<()> {
        let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
        assert_eq!(
            document_path.document_id(),
            &DocumentId::from_str("chatroom1")?
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_document_path_for_collection_path() -> anyhow::Result<()> {
        let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
        assert_eq!(
            CollectionPath::from(document_path),
            CollectionPath::from_str("chatrooms")?
        );
        let document_path = DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?;
        assert_eq!(
            CollectionPath::from(document_path),
            CollectionPath::from_str("chatrooms/chatroom1/messages")?
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_document_path_for_document_id() -> anyhow::Result<()> {
        let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
        assert_eq!(
            DocumentId::from(document_path),
            DocumentId::from_str("chatroom1")?
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_str_and_impl_try_from_string() -> anyhow::Result<()> {
        for (s, expected) in [
            ("chatrooms", false),
            ("chatrooms/chatroom1", true),
            ("chatrooms/chatroom1/messages/message1", true),
        ] {
            assert_eq!(DocumentPath::from_str(s).is_ok(), expected);
            assert_eq!(DocumentPath::try_from(s).is_ok(), expected);
            assert_eq!(DocumentPath::try_from(s.to_string()).is_ok(), expected);
            if expected {
                assert_eq!(DocumentPath::from_str(s)?, DocumentPath::try_from(s)?);
                assert_eq!(
                    DocumentPath::from_str(s)?,
                    DocumentPath::try_from(s.to_string())?
                );
                assert_eq!(DocumentPath::from_str(s)?.to_string(), s);
            }
        }
        Ok(())
    }

    #[test]
    fn test_new() -> anyhow::Result<()> {
        let collection_path = build_collection_path()?;
        let document_id = build_document_id()?;
        let document_path = DocumentPath::new(collection_path.clone(), document_id.clone());
        assert_eq!(
            document_path.to_string(),
            format!("{}/{}", collection_path, document_id)
        );
        Ok(())
    }

    #[test]
    fn test_parent() -> anyhow::Result<()> {
        let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
        assert_eq!(
            document_path.parent(),
            &CollectionPath::from_str("chatrooms")?
        );
        let document_path = DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?;
        assert_eq!(
            document_path.parent(),
            &CollectionPath::from_str("chatrooms/chatroom1/messages")?
        );
        Ok(())
    }

    fn build_collection_path() -> anyhow::Result<CollectionPath> {
        Ok(CollectionPath::from_str("chatrooms")?)
    }

    fn build_document_id() -> anyhow::Result<DocumentId> {
        Ok(DocumentId::from_str("chatroom1")?)
    }
}
