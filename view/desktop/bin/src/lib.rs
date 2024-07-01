mod mem;

pub mod menu;
pub mod service;

use anyhow::Result;
use service::project_service::ProjectService;
use surrealdb::Result as SurrealResult;
use surrealdb::{
    method::Create,
    opt::{self, Resource},
    Connection, Surreal,
};

#[macro_use]
extern crate serde;

#[macro_use]
extern crate tracing;

pub struct AppState {
    pub project_service: ProjectService,
}
