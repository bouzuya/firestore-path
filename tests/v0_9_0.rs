#[test]
fn test_impl_default_for_database_id() {
    // Added: impl Default for DatabaseId
    use firestore_path::DatabaseId;
    assert_eq!(DatabaseId::default().to_string(), "(default)");
}
