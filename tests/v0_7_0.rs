use std::str::FromStr;

use firestore_path::{CollectionName, CollectionPath, DocumentId, DocumentName, DocumentPath};

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
