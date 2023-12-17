use std::str::FromStr;

use crate::{CollectionId, CollectionPath, DocumentId};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("collection id {0}")]
    CollectionId(String),
    #[error("collection path {0}")]
    CollectionPath(#[from] Box<crate::collection_path::Error>),
    #[error("document id {0}")]
    DocumentId(#[from] crate::document_id::Error),
    #[error("todo")]
    ToDo,
}

/// format: `{collection_path}/{document_id}`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DocumentPath {
    collection_path: Box<CollectionPath>,
    document_id: DocumentId,
}

impl DocumentPath {
    pub fn new(collection_path: CollectionPath, document_id: DocumentId) -> Self {
        Self {
            collection_path: Box::new(collection_path),
            document_id,
        }
    }

    pub fn collection<E, T>(self, collection_id: T) -> Result<CollectionPath, Error>
    where
        E: std::fmt::Display,
        T: TryInto<CollectionId, Error = E>,
    {
        let collection_id = collection_id
            .try_into()
            .map_err(|e| Error::CollectionId(e.to_string()))?;
        let collection_path = CollectionPath::new(Some(self), collection_id);
        Ok(collection_path)
    }

    pub fn document_id(&self) -> &DocumentId {
        &self.document_id
    }

    pub fn parent(&self) -> &CollectionPath {
        self.collection_path.as_ref()
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
                collection_path: Box::new(
                    CollectionPath::from_str(collection_path).map_err(Box::new)?,
                ),
                document_id: DocumentId::from_str(document_id)?,
            },
            None => {
                return Err(Error::ToDo);
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
