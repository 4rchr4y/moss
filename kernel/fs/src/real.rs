use anyhow::Result;
use futures::{AsyncRead, Stream};
use notify::Watcher;
use smol::{
    fs::{File, OpenOptions},
    stream::StreamExt,
};
use std::{
    io,
    path::{Path, PathBuf},
    pin::Pin,
    time::Duration,
};

use crate::{file::Metadata, CreateOptions};

#[derive(Debug, Clone)]
pub struct FileSystem;

impl FileSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystem {
    pub async fn open_with_options(
        &self,
        // options: smol::fs::OpenOptions,
        path: &Path,
    ) -> io::Result<File> {
        Ok(OpenOptions::new().write(true).open(path).await?)
    }
}

#[async_trait]
impl super::FS for FileSystem {
    async fn create_dir(&self, path: &Path) -> anyhow::Result<()> {
        Ok(smol::fs::create_dir_all(path).await?)
    }

    async fn create_file(&self, path: &Path, options: CreateOptions) -> anyhow::Result<()> {
        let mut open_options = smol::fs::OpenOptions::new();
        open_options.write(true).create(true);
        if options.overwrite {
            open_options.truncate(true);
        } else if !options.ignore_if_exists {
            open_options.create_new(true);
        }
        open_options.open(path).await?;
        Ok(())
    }

    async fn create_file_with(
        &self,
        path: &Path,
        content: Pin<&mut (dyn AsyncRead + Send)>,
    ) -> anyhow::Result<()> {
        let mut file = smol::fs::File::create(&path).await?;
        futures::io::copy(content, &mut file).await?;
        Ok(())
    }

    async fn read_dir(
        &self,
        path: &Path,
    ) -> anyhow::Result<Pin<Box<dyn Send + Stream<Item = anyhow::Result<PathBuf>>>>> {
        let result = smol::fs::read_dir(path).await?.map(|entry| match entry {
            Ok(entry) => Ok(entry.path()),
            Err(error) => Err(anyhow!("failed to read dir entry {:?}", error)),
        });

        Ok(Box::pin(result))
    }

    async fn read_file(&self, path: &Path) -> anyhow::Result<Box<dyn io::Read>> {
        Ok(Box::new(std::fs::File::open(path)?))
    }

    async fn is_file(&self, path: &Path) -> bool {
        smol::fs::metadata(path)
            .await
            .map_or(false, |metadata| metadata.is_file())
    }

    async fn is_dir(&self, path: &Path) -> bool {
        smol::fs::metadata(path)
            .await
            .map_or(false, |metadata| metadata.is_dir())
    }

    async fn metadata(&self, path: &Path) -> Result<Option<Metadata>> {
        let symlink_metadata = match smol::fs::symlink_metadata(path).await {
            Ok(metadata) => metadata,
            Err(err) => {
                return match (err.kind(), err.raw_os_error()) {
                    (io::ErrorKind::NotFound, _) => Ok(None),
                    (io::ErrorKind::Other, Some(libc::ENOTDIR)) => Ok(None),
                    _ => Err(anyhow::Error::new(err)),
                }
            }
        };

        let is_symlink = symlink_metadata.file_type().is_symlink();
        let metadata = if is_symlink {
            smol::fs::metadata(path).await?
        } else {
            symlink_metadata
        };

        Ok(Some(Metadata {
            modified: metadata.modified().unwrap(), // TODO: avoid unwrap()
            is_symlink,
            is_dir: metadata.file_type().is_dir(),
        }))
    }

    async fn watch(
        &self,
        path: &Path,
        _latency: Duration, // TODO: use this
    ) -> Pin<Box<dyn Send + Stream<Item = notify::Event>>> {
        let (tx, rx) = smol::channel::unbounded();

        let mut file_watcher = notify::recommended_watcher({
            let tx = tx.clone();

            move |event: Result<notify::Event, _>| {
                if let Some(event) = event.ok() {
                    tx.try_send(event).ok();
                }
            }
        })
        .expect("Could not start file watcher");

        file_watcher
            .watch(path, notify::RecursiveMode::Recursive)
            .unwrap();

        let mut parent_watcher = notify::recommended_watcher({
            let watched_path = path.parent().unwrap().to_path_buf();
            let tx = tx.clone();

            move |event: Result<notify::Event, _>| {
                if let Some(event) = event.ok() {
                    if event
                        .clone()
                        .paths
                        .into_iter()
                        .any(|path| path.starts_with(&watched_path))
                    {
                        // println!("Parent event detected: {:?}", event);
                        if let Err(e) = tx.try_send(event) {
                            eprintln!("Error sending event: {:?}", e);
                        }
                    }
                }
            }
        })
        .expect("Could not start file watcher");

        parent_watcher
            .watch(
                path.parent()
                    .expect("Watching root is probably not what you want"),
                notify::RecursiveMode::NonRecursive,
            )
            .expect("Could not start watcher on parent directory");

        Box::pin(rx.chain(futures::stream::once(async move {
            drop(parent_watcher);
            drop(file_watcher);

            notify::Event {
                kind: notify::EventKind::Other,
                paths: vec![],
                attrs: Default::default(),
            }
        })))
    }
}

pub mod types {
    pub enum FileSystemEntity {
        File {
            name: String,
            content: Option<String>,
        },
        Directory {
            name: String,
            children: Option<Vec<FileSystemEntity>>,
        },
    }

    impl FileSystemEntity {
        pub fn name(&self) -> &String {
            match self {
                FileSystemEntity::File { name, .. } => name,
                FileSystemEntity::Directory { name, .. } => name,
            }
        }
    }
}
