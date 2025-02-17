#![allow(missing_docs)]

use std::str::FromStr;

use firestore_path::{
    CollectionId, CollectionName, CollectionPath, DatabaseName, DocumentId, DocumentName,
    DocumentPath, RootDocumentName,
};

#[test]
fn test_collection_name_doc() -> anyhow::Result<()> {
    // BREAKING CHANGE: CollectionName::doc doesn't consume self.
    let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages";
    let collection_name = CollectionName::from_str(s)?;
    assert_eq!(
        collection_name.doc("message1")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    assert_eq!(
        collection_name.doc("message2")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message2"
        )?
    );
    Ok(())
}

#[test]
fn test_collection_name_into_doc() -> anyhow::Result<()> {
    // Added: CollectionName::into_doc
    let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages";
    let collection_name = CollectionName::from_str(s)?;
    assert_eq!(
        collection_name.into_doc("message1")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    Ok(())
}

#[test]
fn test_collection_path_doc() -> anyhow::Result<()> {
    // BREAKING CHANGE: CollectionPath::doc doesn't consume self.
    let s = "chatrooms/chatroom1/messages";
    let collection_path = CollectionPath::from_str(s)?;
    assert_eq!(
        collection_path.doc("message1")?,
        DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?
    );
    assert_eq!(
        collection_path.doc("message2")?,
        DocumentPath::from_str("chatrooms/chatroom1/messages/message2")?
    );
    Ok(())
}

#[test]
fn test_collection_path_into_doc() -> anyhow::Result<()> {
    // Added: CollectionPath::into_doc
    let s = "chatrooms/chatroom1/messages";
    let collection_path = CollectionPath::from_str(s)?;
    assert_eq!(
        collection_path.into_doc("message1")?,
        DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?
    );

    let collection_path = CollectionPath::from_str(s)?;
    assert_eq!(
        collection_path.into_doc("message1".to_string())?,
        DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?
    );

    let collection_path = CollectionPath::from_str(s)?;
    let document_id = DocumentId::from_str("message1")?;
    assert_eq!(
        collection_path.into_doc(document_id)?,
        DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?
    );
    Ok(())
}

#[test]
fn test_database_name_collection() -> anyhow::Result<()> {
    // BREAKING CHANGE: DatabaseName::collection doesn't consume self.
    let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
    assert_eq!(
        database_name.collection("chatrooms")?,
        CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    );
    assert_eq!(
        database_name.collection("chatrooms/chatroom1/messages")?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    assert_eq!(
        database_name.collection(CollectionId::from_str("chatrooms")?)?,
        CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    );
    assert_eq!(
        database_name.collection(CollectionPath::from_str("chatrooms/chatroom1/messages")?)?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    Ok(())
}

#[test]
fn test_database_name_doc() -> anyhow::Result<()> {
    // BREAKING CHANGE: DatabaseName::doc doesn't consume self.
    let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
    assert_eq!(
        database_name.doc("chatrooms/chatroom1")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?
    );
    assert_eq!(
        database_name.doc("chatrooms/chatroom1/messages/message1")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    assert_eq!(
        database_name.doc(DocumentPath::from_str("chatrooms/chatroom1")?)?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?
    );
    assert_eq!(
        database_name.doc(DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?)?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    Ok(())
}

#[test]
fn test_database_name_into_collection() -> anyhow::Result<()> {
    // Added: DatabaseName::into_collection
    let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
    assert_eq!(
        database_name.clone().into_collection("chatrooms")?,
        CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    );
    assert_eq!(
        database_name
            .clone()
            .into_collection("chatrooms/chatroom1/messages")?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    assert_eq!(
        database_name
            .clone()
            .into_collection(CollectionId::from_str("chatrooms")?)?,
        CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    );
    assert_eq!(
        database_name.into_collection(CollectionPath::from_str("chatrooms/chatroom1/messages")?)?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    Ok(())
}

#[test]
fn test_database_name_into_doc() -> anyhow::Result<()> {
    // Added: DatabaseName::into_doc
    let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
    assert_eq!(
        database_name.clone().into_doc("chatrooms/chatroom1")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?
    );
    assert_eq!(
        database_name.clone().into_doc("chatrooms/chatroom1/messages/message1")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    assert_eq!(
        database_name
            .clone()
            .into_doc(DocumentPath::from_str("chatrooms/chatroom1")?)?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?
    );
    assert_eq!(
        database_name.into_doc(DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?)?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    Ok(())
}

#[test]
fn test_document_name_collection() -> anyhow::Result<()> {
    // BREAKING CHANGE: DocumentName::collection doesn't consume self.
    let document_name = DocumentName::from_str(
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
    )?;
    assert_eq!(
        document_name.collection("messages")?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    assert_eq!(
        document_name.collection("messages/message1/col")?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
        )?
    );
    assert_eq!(
        document_name.collection(CollectionId::from_str("messages")?)?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    assert_eq!(
        document_name.collection(CollectionPath::from_str("messages/message1/col")?)?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
        )?
    );
    Ok(())
}

#[test]
fn test_document_name_doc() -> anyhow::Result<()> {
    // BREAKING CHANGE: DocumentName::doc doesn't consume self.
    let document_name = DocumentName::from_str(
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
    )?;
    assert_eq!(
        document_name.doc("messages/message1")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    assert_eq!(
        document_name.doc("messages/message1/col/doc")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col/doc"
        )?
    );
    assert_eq!(
        document_name.doc(DocumentPath::from_str("messages/message1")?)?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    assert_eq!(
        document_name.doc(DocumentPath::from_str("messages/message1/col/doc")?)?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col/doc"
        )?
    );
    Ok(())
}

#[test]
fn test_document_name_into_collection() -> anyhow::Result<()> {
    // Added: DocumentName::into_collection
    let document_name = DocumentName::from_str(
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
    )?;
    assert_eq!(
        document_name.clone().into_collection("messages")?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    assert_eq!(
        document_name.clone().into_collection("messages/message1/col")?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
        )?
    );
    assert_eq!(
        document_name
            .clone()
            .into_collection(CollectionId::from_str("messages")?)?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    assert_eq!(
        document_name.into_collection(CollectionPath::from_str("messages/message1/col")?)?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
        )?
    );
    Ok(())
}

#[test]
fn test_document_name_into_doc() -> anyhow::Result<()> {
    // Added: DocumentName::into_doc
    let document_name = DocumentName::from_str(
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
    )?;
    assert_eq!(
        document_name.clone().into_doc("messages/message1")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    assert_eq!(
        document_name.clone().into_doc("messages/message1/col/doc")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col/doc"
        )?
    );
    assert_eq!(
        document_name.clone().into_doc("messages/message1".to_string())?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    assert_eq!(
        document_name.clone().into_doc(DocumentPath::from_str("messages/message1")?)?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    assert_eq!(
        document_name.into_doc(DocumentPath::from_str("messages/message1/col/doc")?)?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col/doc"
        )?
    );
    Ok(())
}

#[test]
fn test_document_path_collection() -> anyhow::Result<()> {
    // BREAKING CHANGE: DocumentPath::collection doesn't consume self.
    let document_name = DocumentName::from_str(
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
    )?;
    assert_eq!(
        document_name.collection("messages")?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    assert_eq!(
        document_name.collection("messages/message1/col")?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
        )?
    );
    assert_eq!(
        document_name.collection(CollectionId::from_str("messages")?)?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    assert_eq!(
        document_name.collection(CollectionPath::from_str("messages/message1/col")?)?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
        )?
    );
    Ok(())
}

#[test]
fn test_document_path_doc() -> anyhow::Result<()> {
    // BREAKING CHANGE: DocumentPath::doc doesn't consume self.
    let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
    assert_eq!(
        document_path.doc("messages/message1")?,
        DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?
    );
    assert_eq!(
        document_path.doc("messages/message1/col/doc")?,
        DocumentPath::from_str("chatrooms/chatroom1/messages/message1/col/doc")?
    );
    assert_eq!(
        document_path.doc(DocumentPath::from_str("messages/message1")?)?,
        DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?
    );
    assert_eq!(
        document_path.doc(DocumentPath::from_str("messages/message1/col/doc")?)?,
        DocumentPath::from_str("chatrooms/chatroom1/messages/message1/col/doc")?
    );
    Ok(())
}

#[test]
fn test_document_path_into_collection() -> anyhow::Result<()> {
    // Added: DocumentPath::into_collection
    let document_name = DocumentName::from_str(
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
    )?;
    assert_eq!(
        document_name.clone().into_collection("messages")?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    assert_eq!(
        document_name.clone().into_collection("messages/message1/col")?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
        )?
    );
    assert_eq!(
        document_name
            .clone()
            .into_collection(CollectionId::from_str("messages")?)?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    assert_eq!(
        document_name.into_collection(CollectionPath::from_str("messages/message1/col")?)?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
        )?
    );
    Ok(())
}

#[test]
fn test_document_path_into_doc() -> anyhow::Result<()> {
    // Added: DocumentPath::into_doc
    let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
    assert_eq!(
        document_path.clone().into_doc("messages/message1")?,
        DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?
    );
    assert_eq!(
        document_path
            .clone()
            .into_doc("messages/message1/col/doc")?,
        DocumentPath::from_str("chatrooms/chatroom1/messages/message1/col/doc")?
    );
    assert_eq!(
        document_path
            .clone()
            .into_doc("messages/message1".to_string())?,
        DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?
    );
    assert_eq!(
        document_path
            .clone()
            .into_doc(DocumentPath::from_str("messages/message1")?)?,
        DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?
    );
    assert_eq!(
        document_path.into_doc(DocumentPath::from_str("messages/message1/col/doc")?)?,
        DocumentPath::from_str("chatrooms/chatroom1/messages/message1/col/doc")?
    );
    Ok(())
}

#[test]
fn test_root_document_name_collection() -> anyhow::Result<()> {
    // BREAKING CHANGE: RootDocumentName::collection doesn't consume self.
    let root_document_name =
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?;
    assert_eq!(
        root_document_name.collection("chatrooms")?,
        CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    );
    assert_eq!(
        root_document_name.collection("chatrooms/chatroom1/messages")?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    assert_eq!(
        root_document_name.collection(CollectionId::from_str("chatrooms")?)?,
        CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    );
    assert_eq!(
        root_document_name.collection(CollectionPath::from_str("chatrooms/chatroom1/messages")?)?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    Ok(())
}

#[test]
fn test_root_document_name_doc() -> anyhow::Result<()> {
    // BREAKING CHANGE: RootDocumentName::doc doesn't consume self.
    let root_document_name =
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?;
    assert_eq!(
        root_document_name.doc("chatrooms/chatroom1")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?
    );
    assert_eq!(
        root_document_name.doc("chatrooms/chatroom1/messages/message1")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    assert_eq!(
        root_document_name.doc(DocumentPath::from_str("chatrooms/chatroom1")?)?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?
    );
    assert_eq!(
        root_document_name.doc(DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?)?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    Ok(())
}

#[test]
fn test_root_document_name_into_collection() -> anyhow::Result<()> {
    // Added: RootDocumentName::into_collection
    let root_document_name =
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?;
    assert_eq!(
        root_document_name.clone().into_collection("chatrooms")?,
        CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    );
    assert_eq!(
        root_document_name
            .clone()
            .into_collection("chatrooms/chatroom1/messages")?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    assert_eq!(
        root_document_name
            .clone()
            .into_collection(CollectionId::from_str("chatrooms")?)?,
        CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    );
    assert_eq!(
        root_document_name
            .into_collection(CollectionPath::from_str("chatrooms/chatroom1/messages")?)?,
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    Ok(())
}

#[test]
fn test_root_document_name_into_doc() -> anyhow::Result<()> {
    // Added: RootDocumentName::into_doc
    let root_document_name =
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?;
    assert_eq!(
        root_document_name.clone().into_doc("chatrooms/chatroom1")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?
    );
    assert_eq!(
        root_document_name.clone().into_doc("chatrooms/chatroom1/messages/message1")?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    assert_eq!(
        root_document_name
            .clone()
            .into_doc(DocumentPath::from_str("chatrooms/chatroom1")?)?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?
    );
    assert_eq!(
        root_document_name.doc(DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?)?,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
        )?
    );
    Ok(())
}
