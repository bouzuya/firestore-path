use std::str::FromStr;

use firestore_path::{
    CollectionName, CollectionPath, DatabaseName, DocumentName, DocumentPath, RootDocumentName,
};

#[test]
fn test_conversion_database_name_and_string() -> anyhow::Result<()> {
    let s = "projects/my-project/databases/my-database";
    let database_name = DatabaseName::from_str(s)?;
    assert_eq!(database_name.to_string(), s);
    Ok(())
}

#[test]
fn test_root_document_name_new() -> anyhow::Result<()> {
    let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
    let root_document_name = RootDocumentName::new(database_name);
    assert_eq!(
        root_document_name,
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?
    );
    Ok(())
}

#[ignore = "RootDocumentName::collection has been replaced by RootDocumentName::into_collection in v0.7.0"]
#[test]
fn test_root_document_name_collection() -> anyhow::Result<()> {
    // let root_document_name =
    //     RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?;
    // let collection_name = root_document_name.clone().collection("chatrooms")?;
    // assert_eq!(
    //     collection_name,
    //     CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    // );

    // let collection_id = CollectionId::from_str("chatrooms")?;
    // let collection_name = root_document_name.clone().collection(collection_id)?;
    // assert_eq!(
    //     collection_name,
    //     CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    // );

    // let collection_path = CollectionPath::from_str("chatrooms")?;
    // let collection_name = root_document_name.collection(collection_path)?;
    // assert_eq!(
    //     collection_name,
    //     CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    // );
    Ok(())
}

#[ignore = "RootDocumentName::doc has been replaced by RootDocumentName::into_doc in v0.7.0"]
#[test]
fn test_root_document_name_doc() -> anyhow::Result<()> {
    let root_document_name =
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?;
    let document_name = root_document_name.clone().into_doc("chatrooms/chatroom1")?;
    assert_eq!(
        document_name,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?
    );

    let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
    let document_name = root_document_name.into_doc(document_path)?;
    assert_eq!(
        document_name,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?
    );
    Ok(())
}

#[test]
fn test_impl_from_database_name_for_root_document_name() -> anyhow::Result<()> {
    let database_name = DatabaseName::from_str("projects/my-project/databases/my-database")?;
    let root_document_name = RootDocumentName::from(database_name);
    assert_eq!(
        root_document_name,
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?
    );
    Ok(())
}

#[test]
fn test_impl_from_root_document_name_for_database_name() -> anyhow::Result<()> {
    let root_document_name =
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?;
    let database_name = DatabaseName::from(root_document_name);
    assert_eq!(
        database_name,
        DatabaseName::from_str("projects/my-project/databases/my-database")?
    );
    Ok(())
}

#[test]
fn test_impl_try_from_str_for_root_document_name() -> anyhow::Result<()> {
    let root_document_name =
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?;
    assert_eq!(
        root_document_name.to_string(),
        "projects/my-project/databases/my-database/documents"
    );
    Ok(())
}

#[test]
fn test_impl_try_from_string_for_root_document_name() -> anyhow::Result<()> {
    let root_document_name = RootDocumentName::try_from(
        "projects/my-project/databases/my-database/documents".to_string(),
    )?;
    assert_eq!(
        root_document_name,
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?
    );
    Ok(())
}

#[test]
fn test_impl_display_for_root_document_name() -> anyhow::Result<()> {
    let root_document_name =
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?;
    let s = root_document_name.to_string();
    assert_eq!(s, "projects/my-project/databases/my-database/documents");
    Ok(())
}

#[test]
fn test_impl_from_str_for_root_document_name() -> anyhow::Result<()> {
    let root_document_name =
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?;
    assert_eq!(
        root_document_name.to_string(),
        "projects/my-project/databases/my-database/documents"
    );
    Ok(())
}

#[test]
fn test_document_name_new_with_root_document_name() -> anyhow::Result<()> {
    let root_document_name =
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?;
    let document_path = DocumentPath::from_str("chatrooms/chatroom1")?;
    let document_name = DocumentName::new(root_document_name, document_path);
    assert_eq!(
        document_name,
        DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1"
        )?
    );
    Ok(())
}

#[test]
fn test_document_name_root_document_name() -> anyhow::Result<()> {
    let document_name = DocumentName::from_str(
        "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
    )?;
    let root_document_name = document_name.root_document_name();
    assert_eq!(
        root_document_name,
        &RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?
    );
    Ok(())
}

#[test]
fn test_collection_name_new_with_root_document_name() -> anyhow::Result<()> {
    let root_document_name =
        RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?;
    let collection_path = CollectionPath::from_str("chatrooms")?;
    let collection_name = CollectionName::new(root_document_name, collection_path);
    assert_eq!(
        collection_name,
        CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?
    );
    Ok(())
}

#[test]
fn test_collection_name_root_document_name() -> anyhow::Result<()> {
    let collection_name =
        CollectionName::from_str("projects/my-project/databases/my-database/documents/chatrooms")?;
    let root_document_name = collection_name.root_document_name();
    assert_eq!(
        root_document_name,
        &RootDocumentName::from_str("projects/my-project/databases/my-database/documents")?
    );
    Ok(())
}
