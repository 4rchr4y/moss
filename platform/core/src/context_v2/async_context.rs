use anyhow::{Context as _, Result};
use derive_more::{Deref, DerefMut};
use std::{future::Future, rc::Weak};

use crate::executor::{BackgroundExecutor, MainThreadExecutor, Task};

use super::{common, transaction_context::TransactionContext, AnyContext, Context, ContextCell};

#[derive(Deref, DerefMut, Clone)]
pub struct AsyncContext {
    #[deref]
    #[deref_mut]
    pub(super) cell: Weak<ContextCell>,
    pub(super) background_executor: BackgroundExecutor,
    pub(super) main_thread_executor: MainThreadExecutor,
}

unsafe impl Send for AsyncContext {}
unsafe impl Sync for AsyncContext {}

impl<'a> AnyContext for AsyncContext {
    type Output<T> = Result<T>;
    type ReadOutput<'b, T: 'b> = Result<T>;

    fn create_atom<T: super::node::NodeValue>(
        &mut self,
        callback: impl FnOnce(&mut super::atom_context::AtomContext<'_, T>) -> T,
    ) -> Self::Output<super::atom::Atom<T>> {
        let ctx_cell = self
            .cell
            .upgrade()
            .ok_or_else(|| anyhow!("context was released"))?;
        let ctx = &mut ctx_cell.borrow_mut();

        Ok(common::stage_create_atom(ctx, callback))
    }

    fn read_atom<'b, T: super::node::NodeValue>(
        &'b self,
        atom: &super::atom::Atom<T>,
    ) -> Self::ReadOutput<'b, T> {
        let ctx_cell = self.cell.upgrade().context("context was released")?;
        let ctx: &mut Context = &mut ctx_cell.borrow_mut();

        Ok(common::read_atom(ctx, atom).clone())
    }

    fn update_atom<T: super::node::NodeValue, R>(
        &mut self,
        atom: &super::atom::Atom<T>,
        callback: impl FnOnce(&mut T, &mut super::atom_context::AtomContext<'_, T>) -> R,
    ) -> Self::Output<R> {
        let ctx_cell = self.cell.upgrade().context("context was released")?;
        let ctx: &mut Context = &mut ctx_cell.borrow_mut();

        Ok(common::stage_update_atom(ctx, atom, callback))
    }

    fn create_selector<T: super::node::NodeValue>(
        &mut self,
        callback: impl Fn(&mut super::selector_context::SelectorContext<'_, T>) -> T + 'static,
    ) -> Self::Output<super::selector::Selector<T>> {
        let ctx_cell = self.cell.upgrade().context("context was released")?;
        let ctx: &mut Context = &mut ctx_cell.borrow_mut();

        Ok(common::stage_create_selector(ctx, callback))
    }

    fn read_selector<'b, T: super::node::NodeValue>(
        &'b mut self,
        selector: &super::selector::Selector<T>,
    ) -> Self::ReadOutput<'b, T> {
        let ctx_cell = self.cell.upgrade().context("context was released")?;
        let ctx: &mut Context = &mut ctx_cell.borrow_mut();

        Ok(common::resolve_selector(ctx, selector).clone())
    }
}

impl AsyncContext {
    pub fn apply<R>(&self, tx_callback: impl FnOnce(&mut TransactionContext) -> R) -> Result<R> {
        let ctx_cell = self
            .cell
            .upgrade()
            .ok_or_else(|| anyhow!("context was released"))?;
        let ctx = &mut ctx_cell.borrow_mut();

        Ok(ctx.apply(tx_callback))
    }

    pub fn block_on_with<Fut>(&self, f: impl FnOnce(AsyncContext) -> Fut) -> Fut::Output
    where
        Fut: Future + 'static,
        Fut::Output: 'static,
    {
        self.background_executor.block_on(f(self.clone()))
    }

    pub fn block_on<R>(&self, fut: impl Future<Output = R>) -> R {
        self.background_executor.block_on(fut)
    }

    pub fn spawn_local<Fut>(&self, f: impl FnOnce(AsyncContext) -> Fut) -> Task<Fut::Output>
    where
        Fut: Future + 'static,
        Fut::Output: 'static,
    {
        self.main_thread_executor.spawn_local(f(self.clone()))
    }
}
