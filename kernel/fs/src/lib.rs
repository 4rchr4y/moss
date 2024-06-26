pub mod file;
pub mod real;

use anyhow::Result;
use file::Metadata;
use futures::{AsyncRead, Stream};
use std::{
    fmt::Debug,
    io,
    path::{Path, PathBuf},
    pin::Pin,
    time::Duration,
};

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate anyhow;

#[derive(Copy, Clone)]
pub struct CreateOptions {
    pub overwrite: bool,
    pub ignore_if_exists: bool,
}

impl Default for CreateOptions {
    fn default() -> Self {
        Self {
            overwrite: false,
            ignore_if_exists: true,
        }
    }
}

#[async_trait]
pub trait FS: Debug + Send + Sync {
    async fn create_dir(&self, path: &Path) -> anyhow::Result<()>;

    async fn create_file(&self, path: &Path, options: CreateOptions) -> anyhow::Result<()>;

    async fn create_file_with(
        &self,
        path: &Path,
        content: Pin<&mut (dyn AsyncRead + Send)>,
    ) -> anyhow::Result<()>;

    async fn read_dir(
        &self,
        path: &Path,
    ) -> anyhow::Result<Pin<Box<dyn Send + Stream<Item = anyhow::Result<PathBuf>>>>>;

    async fn read_file(&self, path: &Path) -> anyhow::Result<Box<dyn io::Read>>;

    async fn is_file(&self, path: &Path) -> bool;

    async fn is_dir(&self, path: &Path) -> bool;

    async fn metadata(&self, path: &Path) -> Result<Option<Metadata>>;

    async fn watch(
        &self,
        path: &Path,
        latency: Duration,
    ) -> Pin<Box<dyn Send + Stream<Item = notify::Event>>>;
}
