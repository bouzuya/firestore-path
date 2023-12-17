use std::str::FromStr;

use crate::{CollectionId, CollectionPath, DatabaseName, DocumentId, DocumentName, DocumentPath};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("collection path {0}")]
    CollectionPath(#[from] crate::collection_path::Error),
    #[error("database name {0}")]
    DatabaseName(#[from] crate::database_name::Error),
    #[error("document id {0}")]
    DocumentId(String),
    #[error("todo")]
    ToDo,
}

/// format:
/// - `{database_name}/{collection_path}`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CollectionName {
    collection_path: CollectionPath,
    database_name: DatabaseName,
}

impl CollectionName {
    pub fn new(database_name: DatabaseName, collection_path: CollectionPath) -> Self {
        Self {
            collection_path,
            database_name,
        }
    }

    pub fn collection_id(&self) -> &CollectionId {
        self.collection_path.collection_id()
    }

    pub fn doc<E, T>(self, document_id: T) -> Result<DocumentName, Error>
    where
        E: std::fmt::Display,
        T: TryInto<DocumentId, Error = E>,
    {
        let document_id = document_id
            .try_into()
            .map_err(|e| Error::DocumentId(e.to_string()))?;
        let document_path = DocumentPath::new(self.collection_path, document_id);
        let document_name = DocumentName::new(self.database_name, document_path);
        Ok(document_name)
    }

    pub fn parent(self) -> Option<DocumentName> {
        Option::<DocumentPath>::from(self.collection_path)
            .map(|document_path| DocumentName::new(self.database_name, document_path))
    }
}

impl std::convert::From<CollectionName> for CollectionId {
    fn from(collection_name: CollectionName) -> Self {
        Self::from(collection_name.collection_path)
    }
}

impl std::convert::TryFrom<&str> for CollectionName {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // <https://firebase.google.com/docs/firestore/quotas#collections_documents_and_fields>
        if s.len() > 6_144 {
            return Err(Error::ToDo);
        }

        let parts = s.split('/').collect::<Vec<&str>>();
        if parts.len() < 5 + 1 || (parts.len() - 5) % 2 == 0 {
            return Err(Error::ToDo);
        }

        Ok(Self {
            collection_path: CollectionPath::from_str(&parts[5..].join("/"))?,
            database_name: DatabaseName::from_str(&parts[0..5].join("/"))?,
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
        write!(f, "{}/{}", self.database_name, self.collection_path)
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

    use crate::{CollectionId, DatabaseId, ProjectId};

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
    fn test_new() -> anyhow::Result<()> {
        let database_name = build_database_name()?;
        let collection_path = build_collection_path()?;
        let collection_name = CollectionName::new(database_name.clone(), collection_path.clone());
        assert_eq!(
            collection_name.to_string(),
            format!("{}/{}", database_name, collection_path)
        );
        Ok(())
    }

    #[test]
    fn test_parent() -> anyhow::Result<()> {
        let s = "projects/my-project/databases/my-database/documents/chatrooms";
        let collection_name = CollectionName::from_str(s)?;
        assert_eq!(collection_name.parent(), None);

        let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages";
        let collection_name = CollectionName::from_str(s)?;
        assert_eq!(
            collection_name.parent(),
            Some(DocumentName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
            )?)
        );
        Ok(())
    }

    fn build_collection_path() -> anyhow::Result<CollectionPath> {
        let collection_id = CollectionId::from_str("chatrooms")?;
        let collection_path = CollectionPath::new(None, collection_id);
        Ok(collection_path)
    }

    fn build_database_name() -> anyhow::Result<DatabaseName> {
        let project_id = ProjectId::from_str("my-project")?;
        let database_id = DatabaseId::from_str("my-database")?;
        let database_name = DatabaseName::new(project_id, database_id);
        Ok(database_name)
    }
}
