use std::str::FromStr;

use crate::{
    error::ErrorKind, CollectionName, CollectionPath, DatabaseId, DocumentName, DocumentPath,
    Error, ProjectId, RootDocumentName,
};

/// A database name.
///
/// # Format
///
/// `projects/{project_id}/databases/{database_id}`
///
/// # Examples
///
/// ```rust
/// # fn main() -> anyhow::Result<()> {
/// use firestore_path::{DatabaseId,DatabaseName,ProjectId,RootDocumentName};
/// use std::str::FromStr;
///
/// let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
/// assert_eq!(database_name.to_string(), "projects/my-project/databases/my-database");
/// assert_eq!(
///     database_name.root_document_name(),
///     RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?
/// );
///
/// assert_eq!(
///     database_name.database_id(),
///     &DatabaseId::from_str("my-database")?
/// );
/// assert_eq!(
///     database_name.project_id(),
///     &ProjectId::from_str("my-project")?
/// );
///
/// assert_eq!(
///     DatabaseId::from(database_name.clone()),
///     DatabaseId::from_str("my-database")?
/// );
/// assert_eq!(
///     ProjectId::from(database_name.clone()),
///     ProjectId::from_str("my-project")?
/// );
///
/// #     Ok(())
/// # }
/// ```
///
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DatabaseName {
    database_id: DatabaseId,
    project_id: ProjectId,
}

impl DatabaseName {
    /// Creates a new `DatabaseName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DatabaseId,DatabaseName,ProjectId};
    /// use std::str::FromStr;
    ///
    /// let project_id = ProjectId::from_str("my-project")?;
    /// let database_id = DatabaseId::from_str("my-database")?;
    /// let database_name = DatabaseName::new(project_id, database_id);
    /// assert_eq!(database_name.to_string(), "projects/my-project/databases/my-database");
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn new(project_id: ProjectId, database_id: DatabaseId) -> Self {
        Self {
            database_id,
            project_id,
        }
    }

    /// Creates a new `CollectionName` from this `DatabaseName` and `collection_path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,CollectionName,CollectionPath,DatabaseName};
    /// use std::str::FromStr;
    ///
    /// let database_name = DatabaseName::from_str(
    ///     "projects/my-project/databases/my-database"
    /// )?;
    /// assert_eq!(
    ///     database_name.collection("chatrooms")?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms"
    ///     )?
    /// );
    /// assert_eq!(
    ///     database_name.collection("chatrooms/chatroom1/messages")?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
    ///     )?
    /// );
    /// assert_eq!(
    ///     database_name.collection(CollectionId::from_str("chatrooms")?)?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms"
    ///     )?
    /// );
    /// assert_eq!(
    ///     database_name.collection(CollectionPath::from_str("chatrooms/chatroom1/messages")?)?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
    ///     )?
    /// );
    ///
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

    /// Creates a new `CollectionName` by consuming the `DatabaseName` with the provided `collection_path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{CollectionId,CollectionName,CollectionPath,DatabaseName};
    /// use std::str::FromStr;
    ///
    /// let database_name = DatabaseName::from_str(
    ///     "projects/my-project/databases/my-database"
    /// )?;
    /// assert_eq!(
    ///     database_name.clone().into_collection("chatrooms")?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms"
    ///     )?
    /// );
    /// assert_eq!(
    ///     database_name.clone().into_collection("chatrooms/chatroom1/messages")?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
    ///     )?
    /// );
    /// assert_eq!(
    ///     database_name.clone().into_collection(CollectionId::from_str("chatrooms")?)?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms"
    ///     )?
    /// );
    /// assert_eq!(
    ///     database_name.into_collection(CollectionPath::from_str("chatrooms/chatroom1/messages")?)?,
    ///     CollectionName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
    ///     )?
    /// );
    ///
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn into_collection<E, T>(self, collection_path: T) -> Result<CollectionName, Error>
    where
        E: std::fmt::Display,
        T: TryInto<CollectionPath, Error = E>,
    {
        let collection_path = collection_path
            .try_into()
            .map_err(|e| Error::from(ErrorKind::CollectionPathConversion(e.to_string())))?;
        Ok(CollectionName::new(self, collection_path))
    }

    /// Returns the `DatabaseId` of this `DatabaseName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DatabaseId,DatabaseName};
    /// use std::str::FromStr;
    ///
    /// let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
    /// assert_eq!(
    ///     database_name.database_id(),
    ///     &DatabaseId::from_str("my-database")?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn database_id(&self) -> &DatabaseId {
        &self.database_id
    }

    /// Creates a new `DocumentName` from this `DatabaseName` and `document_path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DocumentName,DocumentPath,DatabaseName};
    /// use std::str::FromStr;
    ///
    /// let database_name = DatabaseName::from_str(
    ///     "projects/my-project/databases/my-database"
    /// )?;
    /// assert_eq!(
    ///     database_name.doc("chatrooms/chatroom1")?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     database_name.doc("chatrooms/chatroom1/messages/message1")?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     database_name.doc(DocumentPath::from_str("chatrooms/chatroom1")?)?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     database_name.doc(DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?)?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
    ///     )?
    /// );
    ///
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

    /// Creates a new `DocumentName` by consuming the `DatabaseName` with the provided `document_path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DocumentName,DocumentPath,DatabaseName};
    /// use std::str::FromStr;
    ///
    /// let database_name = DatabaseName::from_str(
    ///     "projects/my-project/databases/my-database"
    /// )?;
    /// assert_eq!(
    ///     database_name.clone().into_doc("chatrooms/chatroom1")?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     database_name.clone().into_doc("chatrooms/chatroom1/messages/message1")?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     database_name.clone().into_doc(DocumentPath::from_str("chatrooms/chatroom1")?)?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    ///     )?
    /// );
    /// assert_eq!(
    ///     database_name.into_doc(DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?)?,
    ///     DocumentName::from_str(
    ///         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
    ///     )?
    /// );
    ///
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn into_doc<E, T>(self, document_path: T) -> Result<DocumentName, Error>
    where
        E: std::fmt::Display,
        T: TryInto<DocumentPath, Error = E>,
    {
        let document_path = document_path
            .try_into()
            .map_err(|e| Error::from(ErrorKind::DocumentPathConversion(e.to_string())))?;
        Ok(DocumentName::new(self, document_path))
    }

    /// Consumes the `DatabaseName`, returning the `RootDocumentName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DatabaseName,RootDocumentName};
    /// use std::str::FromStr;
    ///
    /// let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
    /// assert_eq!(
    ///     database_name.into_root_document_name(),
    ///     RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn into_root_document_name(self) -> RootDocumentName {
        RootDocumentName::new(self)
    }

    /// Returns the `ProjectId` of this `DatabaseName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DatabaseName,ProjectId};
    /// use std::str::FromStr;
    ///
    /// let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
    /// assert_eq!(
    ///     database_name.project_id(),
    ///     &ProjectId::from_str("my-project")?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    /// Returns a new `RootDocumentName` from this `DatabaseName`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> anyhow::Result<()> {
    /// use firestore_path::{DatabaseName,RootDocumentName};
    /// use std::str::FromStr;
    ///
    /// let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
    /// assert_eq!(
    ///     database_name.root_document_name(),
    ///     RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn root_document_name(&self) -> RootDocumentName {
        self.clone().into_root_document_name()
    }
}

impl std::convert::From<DatabaseName> for DatabaseId {
    fn from(database_name: DatabaseName) -> Self {
        database_name.database_id
    }
}

impl std::convert::From<DatabaseName> for ProjectId {
    fn from(database_name: DatabaseName) -> Self {
        database_name.project_id
    }
}

impl std::convert::TryFrom<&str> for DatabaseName {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if !(1..=1_024 * 6).contains(&s.len()) {
            return Err(Error::from(ErrorKind::LengthOutOfBounds));
        }

        let parts = s.split('/').collect::<Vec<&str>>();
        if parts.len() != 4 {
            return Err(Error::from(ErrorKind::InvalidNumberOfPathComponents));
        }
        if parts[0] != "projects" || parts[2] != "databases" {
            return Err(Error::from(ErrorKind::InvalidName));
        }

        let project_id = ProjectId::from_str(parts[1])?;
        let database_id = DatabaseId::from_str(parts[3])?;
        Ok(Self {
            database_id,
            project_id,
        })
    }
}

impl std::convert::TryFrom<String> for DatabaseName {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl std::fmt::Display for DatabaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "projects/{}/databases/{}",
            self.project_id, self.database_id
        )
    }
}

impl std::str::FromStr for DatabaseName {
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
        let s = "projects/my-project/databases/my-database";
        let database_name = DatabaseName::from_str(s)?;
        assert_eq!(database_name.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_impl_from_str_and_impl_try_from_string() -> anyhow::Result<()> {
        for (s, expected) in [
            ("", false),
            ("projects/my-project/databases/my-database", true),
            ("x".repeat(1024 * 6 + 1).as_ref(), false),
            ("p/my-project/databases/my-database", false),
            ("projects/my-project/d/my-database", false),
            ("projects/P/databases/my-database/d", false),
            ("projects/my-project/databases/D", false),
        ] {
            assert_eq!(DatabaseName::from_str(s).is_ok(), expected);
            assert_eq!(DatabaseName::try_from(s).is_ok(), expected);
            assert_eq!(DatabaseName::try_from(s.to_string()).is_ok(), expected);
            if expected {
                assert_eq!(DatabaseName::from_str(s)?, DatabaseName::try_from(s)?);
                assert_eq!(
                    DatabaseName::from_str(s)?,
                    DatabaseName::try_from(s.to_string())?
                );
                assert_eq!(DatabaseName::from_str(s)?.to_string(), s);
            }
        }
        Ok(())
    }

    #[test]
    fn test_new() -> anyhow::Result<()> {
        let project_id = build_project_id()?;
        let database_id = build_database_id()?;
        let database_name = DatabaseName::new(project_id.clone(), database_id.clone());
        assert_eq!(
            database_name.to_string(),
            format!("projects/{}/databases/{}", project_id, database_id)
        );
        Ok(())
    }

    fn build_database_id() -> anyhow::Result<DatabaseId> {
        Ok(DatabaseId::from_str("my-database")?)
    }

    fn build_project_id() -> anyhow::Result<ProjectId> {
        Ok(ProjectId::from_str("my-project")?)
    }
}
