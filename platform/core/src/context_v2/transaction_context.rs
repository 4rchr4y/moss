use derive_more::{Deref, DerefMut};

use super::atom::Atom;
use super::atom_context::AtomContext;
use super::common;
use super::node::{AnyNode, NodeValue};
use super::selector::Selector;
use super::selector_context::SelectorContext;
use super::AnyContext;
use super::Context;

#[derive(Deref, DerefMut)]
pub struct TransactionContext<'a> {
    #[deref]
    #[deref_mut]
    ctx: &'a mut Context,
    depth_value: usize,
}

impl<'a> Drop for TransactionContext<'a> {
    fn drop(&mut self) {
        self.assert_valid_depth_value(self.ctx.batcher.commit_depth.get());

        if self.ctx.batcher.dec_commit_depth() > 0 {
            return;
        } else {
            self.commit();
        }
    }
}

impl<'a> From<&'a mut Context> for TransactionContext<'a> {
    fn from(ctx: &'a mut Context) -> Self {
        let depth_value = ctx.batcher.inc_commit_depth();

        Self { ctx, depth_value }
    }
}

impl<'a> AnyContext for TransactionContext<'a> {
    type Output<T> = T;
    type ReadOutput<'b, T: 'b> = &'b T where 'a: 'b;

    fn create_atom<T: NodeValue>(
        &mut self,
        callback: impl FnOnce(&mut AtomContext<'_, T>) -> T,
    ) -> Self::Output<Atom<T>> {
        common::stage_create_atom(self, callback)
    }

    fn read_atom<'b, T: NodeValue>(&'b self, atom: &Atom<T>) -> Self::ReadOutput<'b, T> {
        self.next_tree().atom_values.read(&atom.key())
    }

    fn update_atom<T: NodeValue, R>(
        &mut self,
        atom: &Atom<T>,
        callback: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Self::Output<R> {
        common::stage_update_atom(self, atom, callback)
    }

    fn create_selector<T: NodeValue>(
        &mut self,
        callback: impl Fn(&mut SelectorContext<'_, T>) -> T + 'static,
    ) -> Self::Output<Selector<T>> {
        common::stage_create_selector(self, callback)
    }

    fn read_selector<'b, T: NodeValue>(
        &'b mut self,
        selector: &Selector<T>,
    ) -> Self::ReadOutput<'b, T> {
        if !self.next_tree().selector_values.lookup(&selector.key()) {
            let computer = self
                .store
                .known_selectors
                .get(&selector.key)
                .unwrap()
                .clone();

            let value =
                unsafe { computer.compute(&mut SelectorContext::new(self, selector.downgrade())) };

            self.apply(|transaction_context| {
                transaction_context
                    .next_tree_mut()
                    .selector_values
                    .insert(selector.key(), value);
            });
        }

        self.next_tree().selector_values.read(&selector.key())
    }
}

impl<'a> TransactionContext<'a> {
    fn assert_valid_depth_value(&self, prev_value: usize) {
        debug_assert!(
            self.depth_value == prev_value,
            "inconsistent decrementation of transaction context, context depth {}, expected {}",
            self.depth_value,
            prev_value
        );
    }
}
