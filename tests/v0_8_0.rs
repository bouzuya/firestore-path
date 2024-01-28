use std::str::FromStr;

use firestore_path::{CollectionName, DocumentName};

#[test]
fn test_collection_name_parent() -> anyhow::Result<()> {
    // BREAKING CHANGE: CollectionName::parent doesn't consume self.
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
