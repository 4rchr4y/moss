use anyhow::Result;
use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use std::{marker::PhantomData, sync::Weak};

use crate::utl::FlattenAnyhowResult;

use super::{
    atom_context::AtomContext,
    node::{AnyNode, NodeImMap, NodeKey, NodeRefCounter, NodeValue, ProtoNode, Slot, WeakNode},
    AnyContext,
};

pub trait AnyAtom<T>: AnyNode<T> {}

impl<T: NodeValue> WeakNode<T, Atom<T>> {
    pub fn upgrade(&self) -> Option<Atom<T>> {
        Atom::upgrade_from(self)
    }

    pub fn update<'a, C, R>(
        &self,
        ctx: &mut C,
        update: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> Result<R>
    where
        C: AnyContext,
        Result<C::Output<R>>: FlattenAnyhowResult<R>,
    {
        FlattenAnyhowResult::flatten(
            self.upgrade()
                .ok_or_else(|| anyhow!("node release"))
                .map(|this| ctx.update_atom(&this, update)),
        )
    }
}

pub struct OnChangeAtomEvent {}

#[derive(Deref, DerefMut)]
pub struct Atom<T: NodeValue> {
    #[deref]
    #[deref_mut]
    p_node: ProtoNode,
    value_typ: PhantomData<T>,
}

impl<T: NodeValue> Clone for Atom<T> {
    fn clone(&self) -> Self {
        Self {
            p_node: self.p_node.clone(),
            value_typ: self.value_typ,
        }
    }
}

impl<T: NodeValue> AnyAtom<T> for Atom<T> {}

impl<T: NodeValue> AnyNode<T> for Atom<T> {
    type Weak = WeakNode<T, Atom<T>>;

    fn key(&self) -> NodeKey {
        self.p_node.key
    }

    fn downgrade(&self) -> Self::Weak {
        WeakNode {
            wp_node: self.p_node.downgrade(),
            value_typ: self.value_typ,
            node_typ: PhantomData::<Atom<T>>,
        }
    }

    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Atom {
            p_node: weak.wp_node.upgrade()?,
            value_typ: weak.value_typ,
        })
    }
}

impl<T: NodeValue> Atom<T> {
    pub(super) fn new(key: NodeKey, rc: Weak<RwLock<NodeRefCounter>>) -> Self {
        Self {
            p_node: ProtoNode::new(key, rc),
            value_typ: PhantomData,
        }
    }

    pub fn update<C, R>(
        &self,
        ctx: &mut C,
        update: impl FnOnce(&mut T, &mut AtomContext<'_, T>) -> R,
    ) -> C::Output<R>
    where
        C: AnyContext,
    {
        ctx.update_atom(self, update)
    }

    pub fn read<'a, C>(&self, ctx: &'a C) -> &'a T
    where
        C: AnyContext<ReadOutput<'a, T> = &'a T>,
    {
        ctx.read_atom(self)
    }
}

#[derive(Deref, DerefMut, Clone)]
pub(super) struct AtomImMap(NodeImMap);

impl AtomImMap {
    pub fn new() -> Self {
        Self(NodeImMap::new())
    }

    pub fn reserve<T>(
        &self,
        create_slot: impl FnOnce(&Self, NodeKey) -> Atom<T>,
    ) -> Slot<T, Atom<T>>
    where
        T: NodeValue,
    {
        let key = self.rc.write().counts.insert(1.into());
        Slot(create_slot(self, key), PhantomData)
    }

    pub fn insert<T, N>(&mut self, slot: Slot<T, N>, value: T) -> N
    where
        T: NodeValue,
        N: AnyNode<T>,
    {
        let atom = slot.0;
        self.values.insert(atom.key(), Box::new(value));

        atom
    }

    pub fn read<T>(&self, key: &NodeKey) -> &T
    where
        T: NodeValue,
    {
        // TODO: add check for valid context

        self.values[key]
            .as_any_ref()
            .downcast_ref::<T>()
            .unwrap_or_else(|| {
                panic!(
                    "cannot read {} node that is being updated",
                    std::any::type_name::<T>()
                )
            })
    }
}
