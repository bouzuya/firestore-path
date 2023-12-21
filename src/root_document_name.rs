use std::str::FromStr;

use crate::{
    error::ErrorKind, CollectionName, CollectionPath, DatabaseId, DatabaseName, DocumentName,
    DocumentPath, Error, ProjectId,
};

/// A root document name.
///
/// # Format
///
/// `{database_name}/documents`
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use firestore_path::RootDocumentName;
/// use std::str::FromStr;
///
/// let root_document_name = RootDocumentName::from_str(
///     "projects/my-project/databases/my-database/documents"
/// )?;
/// assert_eq!(
///     root_document_name.to_string(),
///     "projects/my-project/databases/my-database/documents"
/// );
/// #     Ok(())
/// # }
/// ```
///
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RootDocumentName {
    database_name: DatabaseName,
}

impl RootDocumentName {
    /// Creates a new `RootDocumentName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,CollectionName,CollectionPath,RootDocumentName};
    /// use std::str::FromStr;
    ///
    /// let root_document_name = RootDocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents"
    /// )?;
    /// assert_eq!(
    ///     root_document_name.clone().collection("chatrooms")?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms"
    ///     )?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn new(database_name: DatabaseName) -> Self {
        Self { database_name }
    }

    /// Creates a new `CollectionName` from this `RootDocumentName` and `collection_path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,CollectionName,CollectionPath,RootDocumentName};
    /// use std::str::FromStr;
    ///
    /// let root_document_name = RootDocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents"
    /// )?;
    /// assert_eq!(
    ///     root_document_name.clone().collection("chatrooms")?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms"
    ///     )?
    /// );
    /// assert_eq!(
    ///     root_document_name.clone().collection("chatrooms/chatroom1/messages")?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
    ///     )?
    /// );
    /// assert_eq!(
    ///     root_document_name.clone().collection(CollectionId::from_str("chatrooms")?)?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms"
    ///     )?
    /// );
    /// assert_eq!(
    ///     root_document_name.collection(CollectionPath::from_str("chatrooms/chatroom1/messages")?)?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
    ///     )?
    /// );
    ///
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn collection<E, T>(self, collection_path: T) -> Result<CollectionName, Error>
    where
        E: std::fmt::Display,
        T: TryInto<CollectionPath, Error = E>,
    {
        let collection_path = collection_path
            .try_into()
            .map_err(|e| Error::from(ErrorKind::CollectionPathConversion(e.to_string())))?;
        Ok(CollectionName::new(
            DatabaseName::from(self),
            collection_path,
        ))
    }

    /// Creates a new `DocumentName` from this `RootDocumentName` and `document_path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DocumentName,DocumentPath,RootDocumentName};
    /// use std::str::FromStr;
    ///
    /// let root_document_name = RootDocumentName::from_str(
    ///     "projects/my-project/databases/my-database/documents"
    /// )?;
    /// assert_eq!(
    ///     root_document_name.clone().doc("chatrooms/chatroom1")?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     root_document_name.clone().doc("chatrooms/chatroom1/messages/message1")?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     root_document_name.clone().doc(DocumentPath::from_str("chatrooms/chatroom1")?)?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     root_document_name.doc(DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?)?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
    ///     )?
    /// );
    ///
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn doc<E, T>(self, document_path: T) -> Result<DocumentName, Error>
    where
        E: std::fmt::Display,
        T: TryInto<DocumentPath, Error = E>,
    {
        let document_path = document_path
            .try_into()
            .map_err(|e| Error::from(ErrorKind::DocumentPathConversion(e.to_string())))?;
        Ok(DocumentName::new(DatabaseName::from(self), document_path))
    }
}

impl std::convert::From<DatabaseName> for RootDocumentName {
    fn from(database_name: DatabaseName) -> Self {
        Self { database_name }
    }
}

impl std::convert::From<RootDocumentName> for DatabaseName {
    fn from(root_document_name: RootDocumentName) -> Self {
        root_document_name.database_name
    }
}

impl std::convert::TryFrom<&str> for RootDocumentName {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if !(1..=1_024 * 6).contains(&s.len()) {
            return Err(Error::from(ErrorKind::LengthOutOfBounds));
        }

        let parts = s.split('/').collect::<Vec<&str>>();
        if parts.len() != 5 {
            return Err(Error::from(ErrorKind::InvalidNumberOfPathComponents));
        }
        if parts[0] != "projects" || parts[2] != "databases" || parts[4] != "documents" {
            return Err(Error::from(ErrorKind::InvalidName));
        }

        let project_id = ProjectId::from_str(parts[1])?;
        let database_id = DatabaseId::from_str(parts[3])?;
        let database_name = DatabaseName::new(project_id, database_id);
        Ok(Self { database_name })
    }
}

impl std::convert::TryFrom<String> for RootDocumentName {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl std::fmt::Display for RootDocumentName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            // FIXME: Fix `DatabaseName::to_string` format.
            "{}",
            self.database_name
        )
    }
}

impl std::str::FromStr for RootDocumentName {
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
        let s = "projects/my-project/databases/my-database/documents";
        let root_document_name = RootDocumentName::from_str(s)?;
        assert_eq!(root_document_name.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_impl_from_str_and_impl_try_from_string() -> anyhow::Result<()> {
        for (s, expected) in [
            ("", false),
            ("projects/my-project/databases/my-database/documents", true),
            ("x".repeat(1024 * 6 + 1).as_ref(), false),
            ("p/my-project/databases/my-database/documents", false),
            ("projects/my-project/d/my-database/documents", false),
            ("projects/my-project/databases/my-database/d", false),
            ("projects/P/databases/my-database/d", false),
            ("projects/my-project/databases/D/d", false),
        ] {
            assert_eq!(RootDocumentName::from_str(s).is_ok(), expected);
            assert_eq!(RootDocumentName::try_from(s).is_ok(), expected);
            assert_eq!(RootDocumentName::try_from(s.to_string()).is_ok(), expected);
            if expected {
                assert_eq!(
                    RootDocumentName::from_str(s)?,
                    RootDocumentName::try_from(s)?
                );
                assert_eq!(
                    RootDocumentName::from_str(s)?,
                    RootDocumentName::try_from(s.to_string())?
                );
                assert_eq!(RootDocumentName::from_str(s)?.to_string(), s);
            }
        }
        Ok(())
    }
}
