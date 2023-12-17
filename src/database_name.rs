use std::str::FromStr;

use crate::{CollectionId, CollectionName, CollectionPath, DatabaseId, ProjectId};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("collection id {0}")]
    CollectionId(String),
    #[error("database id {0}")]
    DatabaseId(#[from] crate::database_id::Error),
    #[error("project id {0}")]
    ProjectId(#[from] crate::project_id::Error),
    #[error("todo")]
    ToDo,
}

// format: `projects/{project_id}/databases/{database_id}/documents`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DatabaseName {
    database_id: DatabaseId,
    project_id: ProjectId,
}

impl DatabaseName {
    pub fn new(project_id: ProjectId, database_id: DatabaseId) -> Self {
        Self {
            database_id,
            project_id,
        }
    }

    pub fn collection<E, T>(self, collection_id: T) -> Result<CollectionName, Error>
    where
        E: std::fmt::Display,
        T: TryInto<CollectionId, Error = E>,
    {
        let collection_id = collection_id
            .try_into()
            .map_err(|e| Error::CollectionId(e.to_string()))?;
        let collection_path = CollectionPath::new(None, collection_id);
        let collection_name = CollectionName::new(self, collection_path);
        Ok(collection_name)
    }
}

impl std::convert::TryFrom<&str> for DatabaseName {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() > 1_024 * 6 {
            return Err(Error::ToDo);
        }

        let parts = s.split('/').collect::<Vec<&str>>();
        if parts.len() != 5
            || parts[0] != "projects"
            || parts[2] != "databases"
            || parts[4] != "documents"
        {
            return Err(Error::ToDo);
        }

        let project_id = ProjectId::from_str(parts[1])?;
        let database_id = DatabaseId::from_str(parts[3])?;
        Ok(Self {
            database_id,
            project_id,
        })
    }
}

impl std::convert::TryFrom<String> for DatabaseName {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl std::fmt::Display for DatabaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "projects/{}/databases/{}/documents",
            self.project_id, self.database_id
        )
    }
}

impl std::str::FromStr for DatabaseName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "projects/my-project/databases/my-database/documents";
        let database_name = DatabaseName::from_str(s)?;
        assert_eq!(database_name.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_collection() -> anyhow::Result<()> {
        let database_name =
            DatabaseName::from_str("projects/my-project/databases/my-database/documents")?;
        let collection_name = database_name.collection("chatrooms")?;
        assert_eq!(
            collection_name,
            CollectionName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms"
            )?
        );

        let database_name =
            DatabaseName::from_str("projects/my-project/databases/my-database/documents")?;
        let collection_id = CollectionId::from_str("chatrooms")?;
        let collection_name = database_name.collection(collection_id)?;
        assert_eq!(
            collection_name,
            CollectionName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms"
            )?
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_str_and_impl_try_from_string() -> anyhow::Result<()> {
        for (s, expected) in [
            ("projects/my-project/databases/my-database/documents", true),
            ("x".repeat(1024 * 6 + 1).as_ref(), false),
            ("p/my-project/databases/my-database/documents", false),
            ("projects/my-project/d/my-database/documents", false),
            ("projects/my-project/databases/my-database/d", false),
            ("projects/P/databases/my-database/d", false),
            ("projects/my-project/databases/D/d", false),
        ] {
            assert_eq!(DatabaseName::from_str(s).is_ok(), expected);
            assert_eq!(DatabaseName::try_from(s).is_ok(), expected);
            assert_eq!(DatabaseName::try_from(s.to_string()).is_ok(), expected);
            if expected {
                assert_eq!(DatabaseName::from_str(s)?, DatabaseName::try_from(s)?);
                assert_eq!(
                    DatabaseName::from_str(s)?,
                    DatabaseName::try_from(s.to_string())?
                );
                assert_eq!(DatabaseName::from_str(s)?.to_string(), s);
            }
        }
        Ok(())
    }

    #[test]
    fn test_new() -> anyhow::Result<()> {
        let project_id = build_project_id()?;
        let database_id = build_database_id()?;
        let database_name = DatabaseName::new(project_id.clone(), database_id.clone());
        assert_eq!(
            database_name.to_string(),
            format!(
                "projects/{}/databases/{}/documents",
                project_id, database_id
            )
        );
        Ok(())
    }

    fn build_database_id() -> anyhow::Result<DatabaseId> {
        Ok(DatabaseId::from_str("my-database")?)
    }

    fn build_project_id() -> anyhow::Result<ProjectId> {
        Ok(ProjectId::from_str("my-project")?)
    }
}
