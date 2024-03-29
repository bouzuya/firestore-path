use std::str::FromStr;

use crate::{
    error::ErrorKind, CollectionId, CollectionPath, DatabaseName, DocumentId, DocumentName,
    DocumentPath, Error, RootDocumentName,
};

/// A collection name.
///
/// # Format
///
/// `{root_document_name}/{collection_path}`
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use firestore_path::{CollectionName,CollectionPath};
/// use std::str::FromStr;
///
/// let collection_name = CollectionName::from_str(
///     "projects/my-project/databases/my-database/documents/chatrooms"
/// )?;
/// assert_eq!(
///     collection_name.to_string(),
///     "projects/my-project/databases/my-database/documents/chatrooms"
/// );
///
/// assert_eq!(
///     collection_name.collection_path(),
///     &CollectionPath::from_str("chatrooms")?
/// );
///
/// assert_eq!(
///     CollectionPath::from(collection_name),
///     CollectionPath::from_str("chatrooms")?
/// );
///
/// #     Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CollectionName {
    collection_path: CollectionPath,
    root_document_name: RootDocumentName,
}

impl CollectionName {
    /// Creates a new `CollectionName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionName,CollectionPath,DatabaseName,RootDocumentName};
    /// use std::str::FromStr;
    ///
    /// let root_document_name = RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?;
    /// let collection_path = CollectionPath::from_str("chatrooms")?;
    /// let collection_name = CollectionName::new(root_document_name, collection_path);
    /// assert_eq!(
    ///     collection_name.to_string(),
    ///     "projects/my-project/databases/my-database/documents/chatrooms"
    /// );
    ///
    /// let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
    /// let collection_path = CollectionPath::from_str("chatrooms")?;
    /// let collection_name = CollectionName::new(database_name, collection_path);
    /// assert_eq!(
    ///     collection_name.to_string(),
    ///     "projects/my-project/databases/my-database/documents/chatrooms"
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn new<D>(root_document_name: D, collection_path: CollectionPath) -> Self
    where
        D: Into<RootDocumentName>,
    {
        Self {
            collection_path,
            root_document_name: root_document_name.into(),
        }
    }

    /// Returns the `CollectionId` of this `CollectionName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,CollectionName};
    /// use std::str::FromStr;
    ///
    /// let collection_name = CollectionName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms"
    /// )?;
    /// assert_eq!(
    ///     collection_name.collection_id(),
    ///     &CollectionId::from_str("chatrooms")?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn collection_id(&self) -> &CollectionId {
        self.collection_path.collection_id()
    }

    /// Returns the `CollectionPath` of this `CollectionName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionName,CollectionPath};
    /// use std::str::FromStr;
    ///
    /// let collection_name = CollectionName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms"
    /// )?;
    /// assert_eq!(
    ///     collection_name.collection_path(),
    ///     &CollectionPath::from_str("chatrooms")?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn collection_path(&self) -> &CollectionPath {
        &self.collection_path
    }

    /// Returns the `DatabaseName` of this `CollectionName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DatabaseName,CollectionName};
    /// use std::str::FromStr;
    ///
    /// let collection_name = CollectionName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms"
    /// )?;
    /// assert_eq!(
    ///     collection_name.database_name(),
    ///     &DatabaseName::from_str("projects/my-project/databases/my-database")?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn database_name(&self) -> &DatabaseName {
        self.root_document_name.as_database_name()
    }

    /// Creates a new `DocumentName` from this `CollectionName` and `document_id`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,CollectionName,DocumentName};
    /// use std::str::FromStr;
    ///
    /// let collection_name = CollectionName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms"
    /// )?;
    /// assert_eq!(
    ///     collection_name.doc("chatroom1")?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     collection_name.doc("chatroom2")?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom2"
    ///     )?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn doc<E, T>(&self, document_id: T) -> Result<DocumentName, Error>
    where
        E: std::fmt::Display,
        T: TryInto<DocumentId, Error = E>,
    {
        self.clone().into_doc(document_id)
    }

    /// Creates a new `DocumentName` by consuming the `CollectionName` with the provided `document_id`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,CollectionName,DocumentName};
    /// use std::str::FromStr;
    ///
    /// let collection_name = CollectionName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms"
    /// )?;
    /// assert_eq!(
    ///     collection_name.clone().into_doc("chatroom1")?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     collection_name.into_doc("chatroom2")?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom2"
    ///     )?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn into_doc<E, T>(self, document_id: T) -> Result<DocumentName, Error>
    where
        E: std::fmt::Display,
        T: TryInto<DocumentId, Error = E>,
    {
        let document_id = document_id
            .try_into()
            .map_err(|e| Error::from(ErrorKind::DocumentIdConversion(e.to_string())))?;
        let document_path = DocumentPath::new(self.collection_path, document_id);
        let document_name = DocumentName::new(self.root_document_name, document_path);
        Ok(document_name)
    }

    /// Consumes the `CollectionName`, returning the parent `DocumentName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,CollectionName,DocumentName};
    /// use std::str::FromStr;
    ///
    /// let collection_name = CollectionName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms"
    /// )?;
    /// assert_eq!(collection_name.into_parent(), None);
    ///
    /// let collection_name = CollectionName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
    /// )?;
    /// assert_eq!(
    ///     collection_name.clone().into_parent(),
    ///     Some(DocumentName::from_str(
    ///       "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    ///     )?)
    /// );
    /// assert_eq!(
    ///     collection_name.into_parent(),
    ///     Some(DocumentName::from_str(
    ///       "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    ///     )?)
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn into_parent(self) -> Option<DocumentName> {
        Option::<DocumentPath>::from(self.collection_path).map(|document_path| {
            DocumentName::new(DatabaseName::from(self.root_document_name), document_path)
        })
    }

    /// Consumes the `CollectionName`, returning the `RootDocumentName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionName,RootDocumentName};
    /// use std::str::FromStr;
    ///
    /// let collection_name = CollectionName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms"
    /// )?;
    /// let root_document_name = collection_name.into_root_document_name();
    /// assert_eq!(
    ///     root_document_name,
    ///     RootDocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents"
    ///     )?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn into_root_document_name(self) -> RootDocumentName {
        self.root_document_name
    }

    /// Returns the parent `DocumentName` of this `CollectionName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,CollectionName,DocumentName};
    /// use std::str::FromStr;
    ///
    /// let collection_name = CollectionName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms"
    /// )?;
    /// assert_eq!(collection_name.parent(), None);
    ///
    /// let collection_name = CollectionName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
    /// )?;
    /// assert_eq!(
    ///     collection_name.parent(),
    ///     Some(DocumentName::from_str(
    ///       "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    ///     )?)
    /// );
    /// assert_eq!(
    ///     collection_name.parent(),
    ///     Some(DocumentName::from_str(
    ///       "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    ///     )?)
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn parent(&self) -> Option<DocumentName> {
        self.clone().into_parent()
    }

    /// Returns the `RootDocumentName` of this `CollectionName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionName,RootDocumentName};
    /// use std::str::FromStr;
    ///
    /// let collection_name = CollectionName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms"
    /// )?;
    /// let root_document_name = collection_name.root_document_name();
    /// assert_eq!(
    ///     root_document_name,
    ///     &RootDocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents"
    ///     )?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn root_document_name(&self) -> &RootDocumentName {
        &self.root_document_name
    }
}

impl std::convert::From<CollectionName> for CollectionId {
    fn from(collection_name: CollectionName) -> Self {
        Self::from(collection_name.collection_path)
    }
}

impl std::convert::From<CollectionName> for CollectionPath {
    fn from(collection_name: CollectionName) -> Self {
        collection_name.collection_path
    }
}

impl std::convert::From<CollectionName> for DatabaseName {
    fn from(collection_name: CollectionName) -> Self {
        Self::from(collection_name.root_document_name)
    }
}

impl std::convert::TryFrom<&str> for CollectionName {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // <https://firebase.google.com/docs/firestore/quotas#collections_documents_and_fields>
        if !(1..=6_144).contains(&s.len()) {
            return Err(Error::from(ErrorKind::LengthOutOfBounds));
        }

        let parts = s.split('/').collect::<Vec<&str>>();
        if parts.len() < 5 + 1 || (parts.len() - 5) % 2 == 0 {
            return Err(Error::from(ErrorKind::InvalidNumberOfPathComponents));
        }

        Ok(Self {
            collection_path: CollectionPath::from_str(&parts[5..].join("/"))?,
            root_document_name: RootDocumentName::from_str(&parts[0..5].join("/"))?,
        })
    }
}

impl std::convert::TryFrom<String> for CollectionName {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl std::fmt::Display for CollectionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.root_document_name, self.collection_path)
    }
}

impl std::str::FromStr for CollectionName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::CollectionId;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "projects/my-project/databases/my-database/documents/chatrooms";
        let collection_name = CollectionName::from_str(s)?;
        assert_eq!(collection_name.to_string(), s);

        let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages";
        let collection_name = CollectionName::from_str(s)?;
        assert_eq!(collection_name.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_collection_id() -> anyhow::Result<()> {
        let s = "projects/my-project/databases/my-database/documents/chatrooms";
        let collection_name = CollectionName::from_str(s)?;
        assert_eq!(
            collection_name.collection_id(),
            &CollectionId::from_str("chatrooms")?
        );
        Ok(())
    }

    #[test]
    fn test_doc() -> anyhow::Result<()> {
        let collection_name = CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms",
        )?;
        let document_name = collection_name.doc("chatroom1")?;
        assert_eq!(
            document_name,
            DocumentName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
            )?
        );
        let document_name = collection_name.doc("chatroom2")?;
        assert_eq!(
            document_name,
            DocumentName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom2"
            )?
        );

        let collection_name = CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages",
        )?;
        let document_name = collection_name.doc("message1")?;
        assert_eq!(
            document_name,
            DocumentName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
            )?
        );

        let collection_name = CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms",
        )?;
        let document_id = DocumentId::from_str("chatroom1")?;
        let document_name = collection_name.doc(document_id)?;
        assert_eq!(
            document_name,
            DocumentName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
            )?
        );

        Ok(())
    }

    #[test]
    fn test_impl_from_collection_name_for_collection_id() -> anyhow::Result<()> {
        let s = "projects/my-project/databases/my-database/documents/chatrooms";
        let collection_name = CollectionName::from_str(s)?;
        assert_eq!(
            CollectionId::from(collection_name),
            CollectionId::from_str("chatrooms")?
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_collection_name_for_database_name() -> anyhow::Result<()> {
        let s = "projects/my-project/databases/my-database/documents/chatrooms";
        let collection_name = CollectionName::from_str(s)?;
        assert_eq!(
            DatabaseName::from(collection_name),
            DatabaseName::from_str("projects/my-project/databases/my-database")?
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_str_and_impl_try_from_string() -> anyhow::Result<()> {
        let b = "projects/my-project/databases/my-database/documents";
        let c1 = "x".repeat(1500);
        let d1 = "x".repeat(1500);
        let c2 = "y".repeat(1500);
        let d2 = "y".repeat(1500);
        let c3_ok = "z".repeat(88);
        let c3_err = "z".repeat(88 + 1);
        let s1 = format!("{}/{}/{}/{}/{}/{}", b, c1, d1, c2, d2, c3_ok);
        assert_eq!(s1.len(), 6_144);
        let s2 = format!("{}/{}/{}/{}/{}/{}", b, c1, d1, c2, d2, c3_err);
        assert_eq!(s2.len(), 6_145);
        for (s, expected) in [
            ("", false),
            ("projects/my-project/databases/my-database/documents", false),
            (
                "projects/my-project/databases/my-database/documents/c",
                true,
            ),
            (
                "projects/my-project/databases/my-database/documents/c/d",
                false,
            ),
            (
                "projects/my-project/databases/my-database/documents/c/d/c",
                true,
            ),
            (s1.as_str(), true),
            (s2.as_str(), false),
        ] {
            assert_eq!(CollectionName::from_str(s).is_ok(), expected);
            assert_eq!(CollectionName::try_from(s).is_ok(), expected);
            assert_eq!(CollectionName::try_from(s.to_string()).is_ok(), expected);
            if expected {
                assert_eq!(CollectionName::from_str(s)?, CollectionName::try_from(s)?);
                assert_eq!(
                    CollectionName::from_str(s)?,
                    CollectionName::try_from(s.to_string())?
                );
                assert_eq!(CollectionName::from_str(s)?.to_string(), s);
            }
        }
        Ok(())
    }

    #[test]
    fn test_into_doc() -> anyhow::Result<()> {
        let collection_name = CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms",
        )?;
        let document_name = collection_name.into_doc("chatroom1")?;
        assert_eq!(
            document_name,
            DocumentName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
            )?
        );

        let collection_name = CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages",
        )?;
        let document_name = collection_name.into_doc("message1")?;
        assert_eq!(
            document_name,
            DocumentName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
            )?
        );

        let collection_name = CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms",
        )?;
        let document_id = DocumentId::from_str("chatroom1")?;
        let document_name = collection_name.into_doc(document_id)?;
        assert_eq!(
            document_name,
            DocumentName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
            )?
        );

        Ok(())
    }

    #[test]
    fn test_parent() -> anyhow::Result<()> {
        let s = "projects/my-project/databases/my-database/documents/chatrooms";
        let collection_name = CollectionName::from_str(s)?;
        assert_eq!(collection_name.into_parent(), None);

        let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages";
        let collection_name = CollectionName::from_str(s)?;
        assert_eq!(
            collection_name.into_parent(),
            Some(DocumentName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
            )?)
        );
        Ok(())
    }
}
