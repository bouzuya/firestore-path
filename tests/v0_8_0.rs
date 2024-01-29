use std::str::FromStr;

#[test]
fn test_collection_name_into_root_document_name() -> anyhow::Result<()> {
    // Added: CollectionName::into_root_document_name
    use firestore_path::{CollectionName, RootDocumentName};
    use std::str::FromStr as _;

    let collection_name =
        CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?;
    let root_document_name = collection_name.into_root_document_name();
    assert_eq!(
        root_document_name,
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?
    );
    Ok(())
}

#[test]
fn test_collection_name_parent() -> anyhow::Result<()> {
    // BREAKING CHANGE: CollectionName::parent doesn't consume self.
    use firestore_path::{CollectionName, DocumentName};
    let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages";
    let collection_name = CollectionName::from_str(s)?;
    assert_eq!(
        collection_name.parent(),
        Some(DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?)
    );
    assert_eq!(
        collection_name.parent(),
        Some(DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?)
    );
    Ok(())
}

#[test]
fn test_collection_name_into_parent() -> anyhow::Result<()> {
    // Added: CollectionName::into_parent
    use firestore_path::{CollectionName, DocumentName};
    let s = "projects/my-project/databases/my-database/documents/chatrooms";
    let collection_name = CollectionName::from_str(s)?;
    assert_eq!(collection_name.into_parent(), None);
    let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages";
    let collection_name = CollectionName::from_str(s)?;
    assert_eq!(
        collection_name.clone().into_parent(),
        Some(DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?)
    );
    assert_eq!(
        collection_name.into_parent(),
        Some(DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?)
    );
    Ok(())
}

#[test]
fn test_collection_path_into_parent() -> anyhow::Result<()> {
    // Added: CollectionPath::into_parent
    use firestore_path::{CollectionPath, DocumentPath};
    let collection_name = CollectionPath::from_str("chatrooms")?;
    assert_eq!(collection_name.into_parent(), None);
    let collection_name = CollectionPath::from_str("chatrooms/chatroom1/messages")?;
    assert_eq!(
        collection_name.clone().into_parent(),
        Some(DocumentPath::from_str("chatrooms/chatroom1")?)
    );
    assert_eq!(
        collection_name.into_parent(),
        Some(DocumentPath::from_str("chatrooms/chatroom1")?)
    );
    Ok(())
}

#[test]
fn test_database_name_into_root_document_name() -> anyhow::Result<()> {
    // Added: DatabaseName::into_root_document_name
    use firestore_path::{DatabaseName, RootDocumentName};
    use std::str::FromStr;
    let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
    let root_document_name = database_name.clone().into_root_document_name();
    assert_eq!(
        root_document_name,
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?
    );
    let root_document_name = database_name.into_root_document_name();
    assert_eq!(
        root_document_name,
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?
    );
    Ok(())
}

#[test]
fn test_database_name_root_document_name() -> anyhow::Result<()> {
    // BREAKING CHANGE: DatabaseName::root_document_name doesn't consume self.
    use firestore_path::{DatabaseName, RootDocumentName};
    use std::str::FromStr;
    let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
    let root_document_name = database_name.root_document_name();
    assert_eq!(
        root_document_name,
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?
    );
    let root_document_name = database_name.root_document_name();
    assert_eq!(
        root_document_name,
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?
    );
    Ok(())
}

#[test]
fn test_document_name_into_parent() -> anyhow::Result<()> {
    // Added: DocumentName::into_parent
    use firestore_path::{CollectionName, DocumentName};
    let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1";
    let document_name = DocumentName::from_str(s)?;
    assert_eq!(
        document_name.into_parent(),
        CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    );
    let s =
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1";
    let document_name = DocumentName::from_str(s)?;
    assert_eq!(
        document_name.clone().into_parent(),
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    assert_eq!(
        document_name.into_parent(),
        CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
        )?
    );
    Ok(())
}

#[test]
fn test_document_name_into_parent_document_name() -> anyhow::Result<()> {
    // Added: DocumentName::into_parent_document_name
    use firestore_path::DocumentName;
    let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1";
    let document_name = DocumentName::from_str(s)?;
    assert_eq!(document_name.into_parent_document_name(), None);
    let s =
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1";
    let document_name = DocumentName::from_str(s)?;
    assert_eq!(
        document_name.clone().into_parent_document_name(),
        Some(DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?)
    );
    assert_eq!(
        document_name.into_parent_document_name(),
        Some(DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?)
    );
    Ok(())
}

#[test]
fn test_document_name_into_root_document_name() -> anyhow::Result<()> {
    // Added: DocumentName::into_root_document_name
    use firestore_path::{DocumentName, RootDocumentName};
    use std::str::FromStr;

    let document_name = DocumentName::from_str(
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
    )?;
    let root_document_name = document_name.into_root_document_name();
    assert_eq!(
        root_document_name,
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?
    );
    Ok(())
}

#[test]
fn test_document_name_parent() -> anyhow::Result<()> {
    // BREAKING CHANGE: DocumentName::parent doesn't consume self.
    use firestore_path::{CollectionName, DocumentName};
    let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1";
    let document_name = DocumentName::from_str(s)?;
    assert_eq!(
        document_name.parent(),
        CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    );
    assert_eq!(
        document_name.parent(),
        CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    );
    Ok(())
}

#[test]
fn test_document_name_parent_document_name() -> anyhow::Result<()> {
    // Add: DocumentName::parent_document_name doesn't consume self.
    use firestore_path::DocumentName;
    let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1";
    let document_name = DocumentName::from_str(s)?;
    assert_eq!(document_name.parent_document_name(), None);
    let s =
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1";
    let document_name = DocumentName::from_str(s)?;
    assert_eq!(
        document_name.parent_document_name(),
        Some(DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?)
    );
    assert_eq!(
        document_name.parent_document_name(),
        Some(DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?)
    );
    Ok(())
}

#[test]
fn test_document_path_into_parent() -> anyhow::Result<()> {
    // Added: DocumentPath::into_parent
    use firestore_path::{CollectionPath, DocumentPath};
    let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
    assert_eq!(
        document_path.into_parent(),
        CollectionPath::from_str("chatrooms")?
    );
    let document_path = DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?;
    assert_eq!(
        document_path.clone().into_parent(),
        CollectionPath::from_str("chatrooms/chatroom1/messages")?
    );
    assert_eq!(
        document_path.into_parent(),
        CollectionPath::from_str("chatrooms/chatroom1/messages")?
    );
    Ok(())
}
