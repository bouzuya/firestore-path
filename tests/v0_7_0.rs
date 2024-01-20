use std::str::FromStr;

use firestore_path::{CollectionName, DocumentName};

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
