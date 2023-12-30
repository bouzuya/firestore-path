use std::str::FromStr;

use firestore_path::{
    CollectionName, CollectionPath, DatabaseId, DatabaseName, DocumentName, DocumentPath, ProjectId,
};

#[test]
fn test_collection_name_collection_path() -> anyhow::Result<()> {
    let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages";
    let collection_name = CollectionName::from_str(s)?;
    assert_eq!(
        collection_name.collection_path(),
        &CollectionPath::from_str("chatrooms/chatroom1/messages")?
    );
    Ok(())
}

#[test]
fn test_database_name_database_id() -> anyhow::Result<()> {
    let s = "projects/my-project/databases/my-database";
    let database_name = DatabaseName::from_str(s)?;
    assert_eq!(
        database_name.database_id(),
        &DatabaseId::from_str("my-database")?
    );
    Ok(())
}

#[test]
fn test_database_name_project_id() -> anyhow::Result<()> {
    let s = "projects/my-project/databases/my-database";
    let database_name = DatabaseName::from_str(s)?;
    assert_eq!(
        database_name.project_id(),
        &ProjectId::from_str("my-project")?
    );
    Ok(())
}

#[test]
fn test_document_name_document_path() -> anyhow::Result<()> {
    let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1";
    let document_name = DocumentName::from_str(s)?;
    assert_eq!(
        document_name.document_path(),
        &DocumentPath::from_str("chatrooms/chatroom1")?
    );
    Ok(())
}

#[test]
fn test_impl_from_collection_name_for_collection_path() -> anyhow::Result<()> {
    let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages";
    let collection_name = CollectionName::from_str(s)?;
    assert_eq!(
        CollectionPath::from(collection_name),
        CollectionPath::from_str("chatrooms/chatroom1/messages")?
    );
    Ok(())
}

#[test]
fn test_impl_from_database_name_for_database_id() -> anyhow::Result<()> {
    let s = "projects/my-project/databases/my-database";
    let database_name = DatabaseName::from_str(s)?;
    assert_eq!(
        DatabaseId::from(database_name),
        DatabaseId::from_str("my-database")?
    );
    Ok(())
}

#[test]
fn test_impl_from_database_name_for_project_id() -> anyhow::Result<()> {
    let s = "projects/my-project/databases/my-database";
    let database_name = DatabaseName::from_str(s)?;
    assert_eq!(
        ProjectId::from(database_name),
        ProjectId::from_str("my-project")?
    );
    Ok(())
}

#[test]
fn test_impl_from_document_name_for_document_path() -> anyhow::Result<()> {
    let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1";
    let document_name = DocumentName::from_str(s)?;
    assert_eq!(
        DocumentPath::from(document_name),
        DocumentPath::from_str("chatrooms/chatroom1")?
    );
    Ok(())
}
