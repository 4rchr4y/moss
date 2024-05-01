use async_graphql::{InputObject, SimpleObject};
use common::id::NanoId;

use super::project::ProjectMeta;

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub(crate) struct Session {
    pub id: NanoId,
    pub project_meta: ProjectMeta,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, SimpleObject)]
pub(crate) struct SessionInfo {
    pub id: NanoId,
    pub project_meta_id: NanoId,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub(crate) struct CreateSessionInput {
    pub project_source: String,
}
