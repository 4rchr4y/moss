use common::thing::Thing;
use std::fmt::Debug;

use super::model::project::{CreateProjectInput, Project, RecentProject};

#[async_trait]
pub(crate) trait ProjectRepository: Debug + Send + Sync {
    async fn create_project(&self, input: &CreateProjectInput) -> super::Result<Project>;
    async fn get_project_by_id(&self, id: String) -> super::Result<Option<Project>>;
    async fn delete_by_id(&self, id: String) -> super::Result<Thing>;
    async fn select_resent_list(
        &self,
        start_time: i64,
        limit: u64,
    ) -> super::Result<Vec<RecentProject>>;
}

#[async_trait]
pub(crate) trait WatchListRepository: Debug + Send + Sync {}
