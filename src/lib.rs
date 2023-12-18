//! A Firestore path helper.
//!
//! ```rust
//! use firestore_path::{CollectionId, CollectionName, DatabaseId, DatabaseName, DocumentId, DocumentName, ProjectId};
//! use std::str::FromStr;
//!
//! fn main() -> anyhow::Result<()> {
//!     let project_id = ProjectId::from_str("my-project")?;
//!     let database_id = DatabaseId::from_str("my-database")?;
//!     let database_name = DatabaseName::new(project_id, database_id);
//!
//!     let document_name: DocumentName = database_name
//!         .collection("chatrooms")?
//!         .doc("chatroom1")?
//!         .collection("messages")?
//!         .doc("message1")?;
//!     assert_eq!(
//!         document_name.to_string(),
//!         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1"
//!     );
//!     assert_eq!(document_name.document_id().as_ref(), "message1");
//!
//!     let collection_name: CollectionName = document_name.parent();
//!     assert_eq!(
//!         collection_name.to_string(),
//!         "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
//!     );
//!     assert_eq!(collection_name.collection_id().as_ref(), "messages");
//!
//!     Ok(())
//! }
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
