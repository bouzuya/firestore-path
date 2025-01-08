#![allow(missing_docs)]

#[test]
fn test_impl_default_for_database_id() {
    // Added: impl Default for DatabaseId
    use firestore_path::DatabaseId;
    assert_eq!(DatabaseId::default().to_string(), "(default)");
}

#[test]
fn test_database_name_from_project_id() -> Result<(), firestore_path::Error> {
    use firestore_path::{DatabaseName, ProjectId};
    use std::str::FromStr;

    let database_name = DatabaseName::from_project_id("my-project")?;
    assert_eq!(
        database_name.to_string(),
        "projects/my-project/databases/(default)"
    );
    let project_id = ProjectId::from_str("my-project")?;
    let database_name = DatabaseName::from_project_id(project_id)?;
    assert_eq!(
        database_name.to_string(),
        "projects/my-project/databases/(default)"
    );
    Ok(())
}
