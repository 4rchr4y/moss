mod ignore_list_repo_impl;
mod project_meta_repo_impl;
mod session_repo_impl;

use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::domain::port::rootdb::{ProjectMetaRepository, SessionRepository};

use self::{
    project_meta_repo_impl::ProjectMetaRepositoryImpl, session_repo_impl::SessionRepositoryImpl,
};

pub struct RootSQLiteAdapter {
    project_meta_repo: Arc<dyn ProjectMetaRepository>,
    session_repo: Arc<dyn SessionRepository>,
}

impl RootSQLiteAdapter {
    pub(crate) fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            project_meta_repo: Arc::new(ProjectMetaRepositoryImpl::new(conn.clone())),
            session_repo: Arc::new(SessionRepositoryImpl::new(conn.clone())),
        }
    }

    pub(crate) fn project_meta_repo(&self) -> Arc<dyn ProjectMetaRepository> {
        Arc::clone(&self.project_meta_repo)
    }

    pub(crate) fn session_repo(&self) -> Arc<dyn SessionRepository> {
        Arc::clone(&self.session_repo)
    }
}
