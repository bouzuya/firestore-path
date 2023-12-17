use std::str::FromStr;

use anyhow::Context;
use firestore_path::{
    CollectionId, CollectionName, CollectionPath, DatabaseId, DatabaseName, DocumentId,
    DocumentName, DocumentPath, ProjectId,
};

#[test]
fn test_collection_id_document_id_and_parent() -> anyhow::Result<()> {
    let document_name = DocumentName::from_str(
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1",
    )?;
    assert_eq!(
        document_name.document_id(),
        &DocumentId::from_str("message1")?
    );
    let collection_name = document_name.parent();
    assert_eq!(
        collection_name.collection_id(),
        &CollectionId::from_str("messages")?
    );
    let document_name = collection_name
        .parent()
        .context("subcollection should have parent")?;
    assert_eq!(
        document_name.document_id(),
        &DocumentId::from_str("chatroom1")?
    );
    let collection_name = document_name.parent();
    assert_eq!(
        collection_name.collection_id(),
        &CollectionId::from_str("chatrooms")?
    );

    let document_path = DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?;
    assert_eq!(
        document_path.document_id(),
        &DocumentId::from_str("message1")?
    );
    let collection_path = document_path.parent();
    assert_eq!(
        collection_path.collection_id(),
        &CollectionId::from_str("messages")?
    );
    let document_path = collection_path
        .parent()
        .context("subcollection should have parent")?;
    assert_eq!(
        document_path.document_id(),
        &DocumentId::from_str("chatroom1")?
    );
    let collection_path = document_path.parent();
    assert_eq!(
        collection_path.collection_id(),
        &CollectionId::from_str("chatrooms")?
    );
    Ok(())
}

#[test]
fn test_building_structs_using_collection_and_doc_helpers() -> anyhow::Result<()> {
    let project_id = ProjectId::from_str("my-project")?;
    let database_id = DatabaseId::from_str("my-database")?;
    let database_name = DatabaseName::new(project_id, database_id);
    let collection_name = database_name.collection("chatrooms")?;
    assert_eq!(
        collection_name.to_string(),
        "projects/my-project/databases/my-database/documents/chatrooms"
    );
    let document_name = collection_name.doc("chatroom1")?;
    assert_eq!(
        document_name.to_string(),
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    );
    let collection_name = document_name.collection("messages")?;
    assert_eq!(
        collection_name.to_string(),
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
    );
    let document_name = collection_name.doc("message1")?;
    assert_eq!(
        document_name.to_string(),
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
    );

    let collection_path = CollectionPath::from_str("chatrooms")?;
    assert_eq!(collection_path.to_string(), "chatrooms");
    let document_path = collection_path.doc("chatroom1")?;
    assert_eq!(document_path.to_string(), "chatrooms/chatroom1");
    let collection_path = document_path.collection("messages")?;
    assert_eq!(collection_path.to_string(), "chatrooms/chatroom1/messages");
    let document_path = collection_path.doc("message1")?;
    assert_eq!(
        document_path.to_string(),
        "chatrooms/chatroom1/messages/message1"
    );

    let document_path = CollectionPath::from_str("chatrooms")?
        .doc(DocumentId::from_str("chatroom1")?)?
        .collection(CollectionId::from_str("messages")?)?
        .doc(DocumentId::from_str("message1")?)?;
    assert_eq!(
        document_path.to_string(),
        "chatrooms/chatroom1/messages/message1"
    );
    Ok(())
}

#[test]
fn test_building_structs_using_new_constructor() -> anyhow::Result<()> {
    let project_id = ProjectId::from_str("my-project")?;
    let database_id = DatabaseId::from_str("my-database")?;
    let database_name = DatabaseName::new(project_id, database_id);
    let collection_id = CollectionId::from_str("chatrooms")?;
    let collection_path = CollectionPath::new(None, collection_id);
    let collection_name = CollectionName::new(database_name.clone(), collection_path.clone());
    assert_eq!(
        collection_name.to_string(),
        "projects/my-project/databases/my-database/documents/chatrooms"
    );
    let document_id = DocumentId::from_str("chatroom1")?;
    let document_path = DocumentPath::new(collection_path, document_id);
    let document_name = DocumentName::new(database_name, document_path);
    assert_eq!(
        document_name.to_string(),
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
    );
    Ok(())
}

#[test]
fn test_conversion_between_string() -> anyhow::Result<()> {
    let s = "chatrooms";
    let collection_id = CollectionId::from_str(s)?;
    assert_eq!(collection_id.to_string(), s);

    let s = "projects/my-project/databases/my-database/documents/chatrooms";
    let collection_name = CollectionName::from_str(s)?;
    assert_eq!(collection_name.to_string(), s);

    let s = "chatrooms";
    let collection_path = CollectionPath::from_str(s)?;
    assert_eq!(collection_path.to_string(), s);

    let s = "my-database";
    let database_id = DatabaseId::from_str(s)?;
    assert_eq!(database_id.to_string(), s);

    let s = "projects/my-project/databases/my-database/documents";
    let database_name = DatabaseName::from_str(s)?;
    assert_eq!(database_name.to_string(), s);

    let s = "chatroom1";
    let document_id = DocumentId::from_str(s)?;
    assert_eq!(document_id.to_string(), s);

    let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1";
    let document_name = DocumentName::from_str(s)?;
    assert_eq!(document_name.to_string(), s);

    let s = "chatrooms/chatroom1";
    let document_path = DocumentPath::from_str(s)?;
    assert_eq!(document_path.to_string(), s);

    let s = "my-project";
    let project_id = ProjectId::from_str(s)?;
    assert_eq!(project_id.to_string(), s);

    Ok(())
}
