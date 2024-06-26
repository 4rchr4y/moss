pub mod settings;

mod event;
mod filetree;
mod local;
mod remote;
mod scanner;
mod snapshot;

use anyhow::Result;
use app::{
    context::{AppContext, AsyncAppContext},
    context_compact::AppContextCompact,
};
use fs::FS;
use futures::stream::Stream;
use smol::channel::Receiver as SmolReceiver;
use std::{borrow::Borrow, sync::Arc};

use crate::model::event::SharedWorktreeEvent;

use self::{local::LocalWorktree, remote::RemoteWorktree, settings::LocalWorktreeSettings};

#[derive(Debug)]
pub enum TreeSource {
    Local(Arc<LocalWorktree>),

    #[allow(dead_code)]
    Remote(Arc<RemoteWorktree>),
}

#[derive(Debug)]
pub struct Worktree {
    source: TreeSource,
    event_pool: SmolReceiver<SharedWorktreeEvent>,
}

impl Worktree {
    pub async fn local(
        ctx: &AppContextCompact,
        fs: Arc<dyn FS>,
        settings: &LocalWorktreeSettings,
    ) -> Result<Self> {
        let worktree = LocalWorktree::new(fs, settings).await?;
        let (event_pool_tx, event_pool_rx) = smol::channel::unbounded();

        // let context = app_cell.app.borrow();

        worktree.run(ctx, event_pool_tx).await?;

        Ok(Self {
            source: TreeSource::Local(worktree),
            event_pool: event_pool_rx,
        })
    }

    pub async fn event_stream(&self) -> impl Stream<Item = SharedWorktreeEvent> {
        futures::stream::unfold(self.event_pool.clone(), |receiver| async {
            match receiver.recv().await {
                Ok(event) => Some((event, receiver)),
                Err(e) => {
                    // TODO: send error event to stream instead of logging
                    error!("failed to receive event: {e}");
                    None
                }
            }
        })
    }
}
