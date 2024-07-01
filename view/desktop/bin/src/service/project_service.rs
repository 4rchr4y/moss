use anyhow::Result;
use specta::Type;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};
use types::id::NanoId;

#[derive(Debug, Clone, Type, Deserialize, Serialize)]
pub struct ProjectMeta {
    #[specta(type = String)]
    pub id: Thing,
    pub source: String,
    pub repository: Option<String>,
    #[specta(type = String)]
    pub created_at: i64,
}

pub struct ProjectService {
    conn: Arc<Surreal<Client>>,
}

#[derive(Debug, Clone, Type, Deserialize, Serialize)]
pub struct CreateProjectMetaInput {
    pub source: String,
    pub repository: Option<String>,
}

impl ProjectService {
    pub fn new(conn: Arc<Surreal<Client>>) -> Self {
        Self { conn }
    }

    pub async fn create_project(
        &self,
        input: &CreateProjectMetaInput,
    ) -> Result<Option<ProjectMeta>> {
        let surql = "CREATE ONLY type::table($table) CONTENT {
                source: $input.source,
                repository: $input.repository,
                created_at: time::unix(),
            }";
        let mut result = self
            .conn
            .query(surql)
            .bind(("table", "project_meta"))
            .bind(("input", input))
            .await?;

        Ok(result.take(0)?)
    }

    pub async fn get_project_by_id(&self, id: String) -> Result<Option<ProjectMeta>> {
        let surql = "SELECT * FROM ONLY type::thing($table, $id)";
        let mut result = self
            .conn
            .query(surql)
            .bind(("table", "project_meta"))
            .bind(("id", id))
            .await?;

        Ok(result.take(0)?)
    }

    pub async fn delete_project_by_id(&self, id: String) -> Result<Option<ProjectMeta>> {
        let surql = "DELETE ONLY type::thing($table, $id)";
        let mut result = self
            .conn
            .query(surql)
            .bind(("table", "project_meta"))
            .bind(("id", id))
            .await?;

        Ok(result.take(0)?)
    }
}
