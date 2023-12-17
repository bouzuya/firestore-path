use std::str::FromStr;

use crate::{CollectionId, CollectionName, CollectionPath, DatabaseName, DocumentId, DocumentPath};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database name {0}")]
    DatabaseName(#[from] crate::database_name::Error),
    #[error("document path {0}")]
    DocumentPath(#[from] crate::document_path::Error),
    #[error("todo")]
    ToDo,
}

/// format:
/// - `{database_name}/{document_path}`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DocumentName {
    database_name: DatabaseName,
    document_path: DocumentPath,
}

impl DocumentName {
    pub fn new(database_name: DatabaseName, document_path: DocumentPath) -> Self {
        Self {
            database_name,
            document_path,
        }
    }

    pub fn collection<E, T>(self, collection_id: T) -> Result<CollectionName, Error>
    where
        E: std::fmt::Display,
        T: TryInto<CollectionId, Error = E>,
    {
        Ok(CollectionName::new(
            self.database_name,
            self.document_path.collection(collection_id)?,
        ))
    }

    pub fn document_id(&self) -> &DocumentId {
        self.document_path.document_id()
    }

    pub fn parent(self) -> CollectionName {
        CollectionName::new(self.database_name, CollectionPath::from(self.document_path))
    }
}

impl std::convert::From<DocumentName> for DocumentId {
    fn from(document_name: DocumentName) -> Self {
        Self::from(document_name.document_path)
    }
}

impl std::convert::TryFrom<&str> for DocumentName {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // <https://firebase.google.com/docs/firestore/quotas#collections_documents_and_fields>
        if s.len() > 6_144 {
            return Err(Error::ToDo);
        }

        let parts = s.split('/').collect::<Vec<&str>>();
        if parts.len() < 5 + 2 || (parts.len() - 5) % 2 != 0 {
            return Err(Error::ToDo);
        }

        Ok(Self {
            database_name: DatabaseName::from_str(&parts[0..5].join("/"))?,
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
        write!(f, "{}/{}", self.database_name, self.document_path)
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

    use crate::{CollectionPath, DatabaseId, DocumentId, ProjectId};

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
        let collection_name = document_name.collection("messages")?;
        assert_eq!(
            collection_name,
            CollectionName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
            )?
        );

        let document_name = DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1",
        )?;
        let collection_name = document_name.collection("col")?;
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
        let collection_name = document_name.collection(collection_id)?;
        assert_eq!(
            collection_name,
            CollectionName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
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
    fn test_new() -> anyhow::Result<()> {
        let database_name = build_database_name()?;
        let document_path = build_document_path()?;
        let document_name = DocumentName::new(database_name.clone(), document_path.clone());
        assert_eq!(
            document_name.to_string(),
            format!("{}/{}", database_name, document_path)
        );
        Ok(())
    }

    #[test]
    fn test_parent() -> anyhow::Result<()> {
        let document_name = DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
        )?;
        assert_eq!(
            document_name.parent(),
            CollectionName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms",
            )?
        );
        Ok(())
    }

    fn build_document_path() -> anyhow::Result<DocumentPath> {
        let collection_path = CollectionPath::from_str("chatrooms")?;
        let document_id = DocumentId::from_str("chatroom1")?;
        let document_path = DocumentPath::new(collection_path, document_id);
        Ok(document_path)
    }

    fn build_database_name() -> anyhow::Result<DatabaseName> {
        let project_id = ProjectId::from_str("my-project")?;
        let database_id = DatabaseId::from_str("my-database")?;
        let database_name = DatabaseName::new(project_id, database_id);
        Ok(database_name)
    }
}
