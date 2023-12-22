use std::{collections::BTreeMap, str::FromStr as _};

use firestore_path::{DatabaseName, DocumentName};
use google_api_proto::google::firestore::v1::{
    precondition::ConditionType, BeginTransactionRequest, CreateDocumentRequest,
    DeleteDocumentRequest, Document, Precondition,
};

#[test]
fn test_begin_transaction_request() -> anyhow::Result<()> {
    let s = "projects/my-project/databases/(default)";
    let database_name = DatabaseName::from_str(s)?;

    let request = BeginTransactionRequest {
        database: database_name.to_string(),
        options: None,
    };

    assert_eq!(request.database, s);
    Ok(())
}

#[test]
fn test_create_document_request() -> anyhow::Result<()> {
    for (s, p, c, d) in [
        (
            "projects/my-project/databases/(default)/documents/chatrooms/chatroom1",
            "projects/my-project/databases/(default)/documents",
            "chatrooms",
            "chatroom1",
        ),
        (
            "projects/my-project/databases/(default)/documents/chatrooms/chatroom1/messages/message1",
            "projects/my-project/databases/(default)/documents/chatrooms/chatroom1",
            "messages",
            "message1",
        ),
    ] {
        let document_name = DocumentName::from_str(s)?;

        let request = CreateDocumentRequest {
            parent: document_name
                .clone()
                .parent()
                .parent()
                .map(|parent_document_name| parent_document_name.to_string())
                .unwrap_or_else(|| document_name.root_document_name().to_string()),
            collection_id: document_name.collection_id().to_string(),
            document_id: document_name.document_id().to_string(),
            document: Some(Document {
                name: "".to_string(),
                fields: BTreeMap::new(),
                create_time: None,
                update_time: None,
            }),
            mask: None,
        };

        assert_eq!(request.parent, p);
        assert_eq!(request.collection_id, c);
        assert_eq!(request.document_id, d);
    }

    Ok(())
}

#[test]
fn test_delete_document_request() -> anyhow::Result<()> {
    for s in [
        "projects/my-project/databases/(default)/documents/chatrooms/chatroom1",
        "projects/my-project/databases/(default)/documents/chatrooms/chatroom1/messages/message1",
    ] {
        let document_name = DocumentName::from_str(s)?;

        let request = DeleteDocumentRequest {
            name: document_name.to_string(),
            current_document: Some(Precondition {
                condition_type: Some(ConditionType::Exists(false)),
            }),
        };

        assert_eq!(request.name, s);
    }

    Ok(())
}
