//! A Firestore path helper.
//!
//! ```rust
//! # fn main() -> anyhow::Result<()> {
//! use firestore_path::{CollectionId, CollectionName, CollectionPath, DatabaseId, DatabaseName, DocumentId, DocumentName, DocumentPath, ProjectId, RootDocumentName};
//! use std::str::FromStr;
//!
//! let project_id = ProjectId::from_str("my-project")?;
//! let database_id = DatabaseId::from_str("my-database")?;
//! let database_name = DatabaseName::new(project_id, database_id);
//! assert_eq!(database_name.to_string(), "projects/my-project/databases/my-database");
//! assert_eq!(
//!     DatabaseName::from_project_id("my-project")?.to_string(),
//!     "projects/my-project/databases/(default)"
//! );
//!
//! let root_document_name: RootDocumentName = database_name.root_document_name();
//! assert_eq!(root_document_name.to_string(), "projects/my-project/databases/my-database/documents");
//!
//! let collection_name: CollectionName = root_document_name.collection("chatrooms")?;
//! assert_eq!(collection_name.to_string(), "projects/my-project/databases/my-database/documents/chatrooms");
//! assert_eq!(collection_name.collection_id().as_ref(), "chatrooms");
//!
//! let document_name: DocumentName = collection_name.doc("chatroom1")?;
//! assert_eq!(document_name.to_string(), "projects/my-project/databases/my-database/documents/chatrooms/chatroom1");
//! assert_eq!(document_name.collection_id().as_ref(), "chatrooms");
//! assert_eq!(document_name.document_id().as_ref(), "chatroom1");
//!
//! let collection_id = CollectionId::from_str("messages")?;
//! let collection_path = CollectionPath::from(collection_id);
//! assert_eq!(collection_path.to_string(), "messages");
//!
//! let document_id = DocumentId::from_str("message1")?;
//! let document_path: DocumentPath = collection_path.doc(document_id)?;
//! assert_eq!(document_path.to_string(), "messages/message1");
//!
//! let child_document_name = document_name.doc(document_path)?;
//! assert_eq!(
//!     child_document_name.to_string(),
//!     "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
//! );
//! #     Ok(())
//! # }
//! ```
mod collection_id;
mod collection_name;
mod collection_path;
mod database_id;
mod database_name;
mod document_id;
mod document_name;
mod document_path;
mod error;
mod project_id;
mod root_document_name;

pub use self::collection_id::CollectionId;
pub use self::collection_name::CollectionName;
pub use self::collection_path::CollectionPath;
pub use self::database_id::DatabaseId;
pub use self::database_name::DatabaseName;
pub use self::document_id::DocumentId;
pub use self::document_name::DocumentName;
pub use self::document_path::DocumentPath;
pub use self::error::Error;
pub use self::project_id::ProjectId;
pub use self::root_document_name::RootDocumentName;
