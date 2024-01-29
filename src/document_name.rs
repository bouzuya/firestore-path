use std::str::FromStr;

use crate::{
    error::ErrorKind, CollectionId, CollectionName, CollectionPath, DatabaseName, DocumentId,
    DocumentPath, Error, RootDocumentName,
};

/// A document name.
///
/// # Format
///
/// `{database_name}/{document_path}`
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use firestore_path::DocumentName;
/// # use firestore_path::{CollectionId,CollectionName,DatabaseName,DocumentId,DocumentPath};
/// # use std::str::FromStr;
///
/// let document_name = DocumentName::from_str(
///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
/// )?;
/// assert_eq!(
///     document_name.to_string(),
///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
/// );
///
/// assert_eq!(
///     document_name.clone().collection("messages")?,
///     CollectionName::from_str(
///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
///     )?
/// );
/// assert_eq!(document_name.collection_id(), &CollectionId::from_str("chatrooms")?);
/// assert_eq!(
///     document_name.database_name(),
///     &DatabaseName::from_str("projects/my-project/databases/my-database")?
/// );
/// assert_eq!(document_name.document_id(), &DocumentId::from_str("chatroom1")?);
/// assert_eq!(document_name.document_path(), &DocumentPath::from_str("chatrooms/chatroom1")?);
/// assert_eq!(
///     document_name.clone().parent(),
///     CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
/// );
///
/// assert_eq!(
///     DocumentPath::from(document_name.clone()),
///     DocumentPath::from_str("chatrooms/chatroom1")?
/// );
///
/// #     Ok(())
/// # }
/// ```
///
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DocumentName {
    document_path: DocumentPath,
    root_document_name: RootDocumentName,
}

impl DocumentName {
    /// Creates a new `DocumentName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DatabaseName,DocumentName,DocumentPath,RootDocumentName};
    /// use std::str::FromStr;
    ///
    /// let root_document_name = RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?;
    /// let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
    /// let document_name = DocumentName::new(root_document_name, document_path);
    /// assert_eq!(
    ///     document_name.to_string(),
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// );
    ///
    /// let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
    /// let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
    /// let document_name = DocumentName::new(database_name, document_path);
    /// assert_eq!(
    ///     document_name.to_string(),
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn new<D>(root_document_name: D, document_path: DocumentPath) -> Self
    where
        D: Into<RootDocumentName>,
    {
        Self {
            document_path,
            root_document_name: root_document_name.into(),
        }
    }

    /// Creates a new `CollectionName` from this `DocumentName` and `collection_path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,CollectionName,CollectionPath,DocumentName};
    /// use std::str::FromStr;
    ///
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// )?;
    /// assert_eq!(
    ///     document_name.collection("messages")?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
    ///     )?
    /// );
    /// assert_eq!(
    ///     document_name.collection("messages/message1/col")?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
    ///     )?
    /// );
    /// assert_eq!(
    ///     document_name.collection(CollectionId::from_str("messages")?)?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
    ///     )?
    /// );
    /// assert_eq!(
    ///     document_name.collection(CollectionPath::from_str("messages/message1/col")?)?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
    ///     )?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn collection<E, T>(&self, collection_path: T) -> Result<CollectionName, Error>
    where
        E: std::fmt::Display,
        T: TryInto<CollectionPath, Error = E>,
    {
        self.clone().into_collection(collection_path)
    }

    /// Returns the `CollectionId` of this `DocumentName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,DocumentName};
    /// use std::str::FromStr;
    ///
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// )?;
    /// assert_eq!(
    ///     document_name.collection_id(),
    ///     &CollectionId::from_str("chatrooms")?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn collection_id(&self) -> &CollectionId {
        self.document_path.collection_id()
    }

    /// Returns the `DatabaseName` of this `DocumentName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DatabaseName,DocumentName,DocumentPath};
    /// use std::str::FromStr;
    ///
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// )?;
    /// assert_eq!(
    ///     document_name.database_name(),
    ///     &DatabaseName::from_str("projects/my-project/databases/my-database")?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn database_name(&self) -> &DatabaseName {
        self.root_document_name.as_database_name()
    }

    /// Creates a new `DocumentName` from this `DocumentName` and `document_path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionPath,DocumentName,DocumentPath};
    /// use std::str::FromStr;
    ///
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// )?;
    /// assert_eq!(
    ///     document_name.doc("messages/message1")?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     document_name.doc("messages/message1/col/doc")?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col/doc"
    ///     )?
    /// );
    /// assert_eq!(
    ///     document_name.doc(DocumentPath::from_str("messages/message1")?)?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     document_name.doc(DocumentPath::from_str("messages/message1/col/doc")?)?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col/doc"
    ///     )?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn doc<E, T>(&self, document_path: T) -> Result<DocumentName, Error>
    where
        E: std::fmt::Display,
        T: TryInto<DocumentPath, Error = E>,
    {
        self.clone().into_doc(document_path)
    }

    /// Returns the `DocumentId` of this `DocumentName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DocumentId,DocumentName};
    /// use std::str::FromStr;
    ///
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// )?;
    /// assert_eq!(document_name.document_id(), &DocumentId::from_str("chatroom1")?);
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn document_id(&self) -> &DocumentId {
        self.document_path.document_id()
    }

    /// Returns the `DocumentPath` of this `DocumentName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DocumentName,DocumentPath};
    /// use std::str::FromStr;
    ///
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// )?;
    /// assert_eq!(
    ///     document_name.document_path(),
    ///     &DocumentPath::from_str("chatrooms/chatroom1")?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn document_path(&self) -> &DocumentPath {
        &self.document_path
    }

    /// Creates a new `CollectionName` from this `DocumentName` and `collection_path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,CollectionName,CollectionPath,DocumentName};
    /// use std::str::FromStr;
    ///
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// )?;
    /// assert_eq!(
    ///     document_name.clone().into_collection("messages")?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
    ///     )?
    /// );
    /// assert_eq!(
    ///     document_name.clone().into_collection("messages/message1/col")?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
    ///     )?
    /// );
    /// assert_eq!(
    ///     document_name.clone().into_collection(CollectionId::from_str("messages")?)?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
    ///     )?
    /// );
    /// assert_eq!(
    ///     document_name.into_collection(CollectionPath::from_str("messages/message1/col")?)?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
    ///     )?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn into_collection<E, T>(self, collection_path: T) -> Result<CollectionName, Error>
    where
        E: std::fmt::Display,
        T: TryInto<CollectionPath, Error = E>,
    {
        Ok(CollectionName::new(
            self.root_document_name,
            self.document_path.into_collection(collection_path)?,
        ))
    }

    /// Creates a new `DocumentName` by consuming the `DocumentName` with the provided `document_path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionPath,DocumentName,DocumentPath};
    /// use std::str::FromStr;
    ///
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// )?;
    /// assert_eq!(
    ///     document_name.clone().into_doc("messages/message1")?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     document_name.clone().into_doc("messages/message1/col/doc")?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col/doc"
    ///     )?
    /// );
    /// assert_eq!(
    ///     document_name.clone().into_doc(DocumentPath::from_str("messages/message1")?)?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     document_name.into_doc(DocumentPath::from_str("messages/message1/col/doc")?)?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col/doc"
    ///     )?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn into_doc<E, T>(self, document_path: T) -> Result<DocumentName, Error>
    where
        E: std::fmt::Display,
        T: TryInto<DocumentPath, Error = E>,
    {
        Ok(DocumentName::new(
            self.root_document_name,
            self.document_path.into_doc(document_path)?,
        ))
    }

    /// Consumes the `DocumentName`, returning the parent `CollectionName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionName,DocumentName};
    /// use std::str::FromStr;
    ///
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// )?;
    /// assert_eq!(
    ///     document_name.into_parent(),
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms"
    ///     )?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn into_parent(self) -> CollectionName {
        CollectionName::new(
            self.root_document_name,
            CollectionPath::from(self.document_path),
        )
    }

    /// Consumes the `DocumentName`, returning the parent `DocumentName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionName,DocumentName};
    /// use std::str::FromStr;
    ///
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// )?;
    /// assert_eq!(document_name.into_parent_document_name(), None);
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
    /// )?;
    /// assert_eq!(
    ///     document_name.into_parent_document_name(),
    ///     Some(DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    ///     )?)
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn into_parent_document_name(self) -> Option<DocumentName> {
        self.document_path
            .into_parent()
            .into_parent()
            .map(|document_path| DocumentName::new(self.root_document_name, document_path))
    }

    /// Returns the parent `CollectionName` of this `DocumentName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionName,DocumentName};
    /// use std::str::FromStr;
    ///
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// )?;
    /// assert_eq!(
    ///     document_name.parent(),
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms"
    ///     )?
    /// );
    /// assert_eq!(
    ///     document_name.parent(),
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms"
    ///     )?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn parent(&self) -> CollectionName {
        self.clone().into_parent()
    }

    /// Returns the parent `DocumentName` of this `DocumentName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionName,DocumentName};
    /// use std::str::FromStr;
    ///
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// )?;
    /// assert_eq!(document_name.parent_document_name(), None);
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
    /// )?;
    /// assert_eq!(
    ///     document_name.parent_document_name(),
    ///     Some(DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    ///     )?)
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn parent_document_name(&self) -> Option<DocumentName> {
        self.clone().into_parent_document_name()
    }

    /// Returns the `RootDocumentName` of this `DocumentName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DocumentName,RootDocumentName};
    /// use std::str::FromStr;
    ///
    /// let document_name = DocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    /// )?;
    /// let root_document_name = document_name.root_document_name();
    /// assert_eq!(
    ///     root_document_name,
    ///     &RootDocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents"
    ///     )?
    /// );
    /// #     Ok(())
    /// # }
    pub fn root_document_name(&self) -> &RootDocumentName {
        &self.root_document_name
    }
}

impl std::convert::From<DocumentName> for DatabaseName {
    fn from(document_name: DocumentName) -> Self {
        Self::from(document_name.root_document_name)
    }
}

impl std::convert::From<DocumentName> for DocumentId {
    fn from(document_name: DocumentName) -> Self {
        Self::from(document_name.document_path)
    }
}

impl std::convert::From<DocumentName> for DocumentPath {
    fn from(document_name: DocumentName) -> Self {
        document_name.document_path
    }
}

impl std::convert::TryFrom<&str> for DocumentName {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // <https://firebase.google.com/docs/firestore/quotas#collections_documents_and_fields>
        if !(1..=6_144).contains(&s.len()) {
            return Err(Error::from(ErrorKind::LengthOutOfBounds));
        }

        let parts = s.split('/').collect::<Vec<&str>>();
        if parts.len() < 5 + 2 || (parts.len() - 5) % 2 != 0 {
            return Err(Error::from(ErrorKind::InvalidNumberOfPathComponents));
        }

        Ok(Self {
            root_document_name: RootDocumentName::from_str(&parts[0..5].join("/"))?,
            document_path: DocumentPath::from_str(&parts[5..].join("/"))?,
        })
    }
}

impl std::convert::TryFrom<String> for DocumentName {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl std::fmt::Display for DocumentName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.root_document_name, self.document_path)
    }
}

impl std::str::FromStr for DocumentName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{CollectionPath, DocumentId};

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1";
        let document_name = DocumentName::from_str(s)?;
        assert_eq!(document_name.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_collection() -> anyhow::Result<()> {
        let document_name = DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
        )?;
        let collection_name = document_name.into_collection("messages")?;
        assert_eq!(
            collection_name,
            CollectionName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
            )?
        );

        let document_name = DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1",
        )?;
        let collection_name = document_name.into_collection("col")?;
        assert_eq!(
            collection_name,
            CollectionName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
            )?
        );

        let document_name = DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
        )?;
        let collection_id = CollectionId::from_str("messages")?;
        let collection_name = document_name.into_collection(collection_id)?;
        assert_eq!(
            collection_name,
            CollectionName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
            )?
        );
        Ok(())
    }

    #[test]
    fn test_collection_with_colleciton_path() -> anyhow::Result<()> {
        let document_name = DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
        )?;
        let collection_name = document_name.into_collection("messages/message1/col")?;
        assert_eq!(
            collection_name,
            CollectionName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
            )?
        );

        let document_name = DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
        )?;
        let collection_path = CollectionPath::from_str("messages/message1/col")?;
        let collection_name = document_name.into_collection(collection_path)?;
        assert_eq!(
            collection_name,
            CollectionName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
            )?
        );
        Ok(())
    }

    #[test]
    fn test_document_id() -> anyhow::Result<()> {
        let document_name = DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
        )?;
        assert_eq!(
            document_name.document_id(),
            &DocumentId::from_str("chatroom1")?
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_database_name_for_document_id() -> anyhow::Result<()> {
        let document_name = DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
        )?;
        assert_eq!(
            DatabaseName::from(document_name),
            DatabaseName::from_str("projects/my-project/databases/my-database")?
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_document_name_for_document_id() -> anyhow::Result<()> {
        let document_name = DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
        )?;
        assert_eq!(
            DocumentId::from(document_name),
            DocumentId::from_str("chatroom1")?
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
        let c3 = "z".repeat(80);
        let d3_ok = "z".repeat(7);
        let d3_err = "z".repeat(7 + 1);
        let s1 = format!("{}/{}/{}/{}/{}/{}/{}", b, c1, d1, c2, d2, c3, d3_ok);
        assert_eq!(s1.len(), 6_144);
        let s2 = format!("{}/{}/{}/{}/{}/{}/{}", b, c1, d1, c2, d2, c3, d3_err);
        assert_eq!(s2.len(), 6_145);

        for (s, expected) in [
            ("projects/my-project/databases/my-database/documents", false),
            (
                "projects/my-project/databases/my-database/documents/c",
                false,
            ),
            (
                "projects/my-project/databases/my-database/documents/c/d",
                true,
            ),
            (
                "projects/my-project/databases/my-database/documents/c/d/c",
                false,
            ),
            (
                "projects/my-project/databases/my-database/documents/c/d/c/d",
                true,
            ),
            (s1.as_ref(), true),
            (s2.as_ref(), false),
        ] {
            assert_eq!(DocumentName::from_str(s).is_ok(), expected);
            assert_eq!(DocumentName::try_from(s).is_ok(), expected);
            assert_eq!(DocumentName::try_from(s.to_string()).is_ok(), expected);
            if expected {
                assert_eq!(DocumentName::from_str(s)?, DocumentName::try_from(s)?);
                assert_eq!(
                    DocumentName::from_str(s)?,
                    DocumentName::try_from(s.to_string())?
                );
                assert_eq!(DocumentName::from_str(s)?.to_string(), s);
            }
        }
        Ok(())
    }

    #[test]
    fn test_parent() -> anyhow::Result<()> {
        let document_name = DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
        )?;
        assert_eq!(
            document_name.into_parent(),
            CollectionName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms",
            )?
        );
        Ok(())
    }
}
