use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use std::{marker::PhantomData, ptr::NonNull, rc::Rc, sync::Weak};

use super::{
    node::{AnyNode, NodeImMap, NodeKey, NodeRefCounter, NodeValue, ProtoNode, Slot, WeakNode},
    selector_context::SelectorContext,
    AnyContext, Context,
};

/// Marker structure for type erasure.
/// Used to cast different types to a common abstract type.
struct Abstract(());

/// Represents a computer that can store and invoke selector
/// computers with different types.
pub(super) struct Computer {
    /// Pointer to the stored callback.
    data: NonNull<Abstract>,
    /// Function pointer to call the stored callback.
    call: unsafe fn(NonNull<Abstract>, NonNull<Abstract>, NonNull<Abstract>),
    /// Function pointer to drop the stored callback.
    drop: unsafe fn(NonNull<Abstract>),
    /// PhantomData to prevent `Send` and `Sync` implementations.
    not_send: PhantomData<Rc<()>>,
}

impl Drop for Computer {
    fn drop(&mut self) {
        unsafe {
            (self.drop)(self.data);
        }
    }
}

impl Computer {
    /// Creates a new `Computer` with the provided callback.
    /// This function uses `unsafe` code to perform type erasure and manage memory manually.
    pub(super) fn new<R, F>(f: F) -> Self
    where
        R: NodeValue,
        F: Fn(&mut SelectorContext<'_, R>) -> R + 'static,
    {
        // The function that calls the stored callback.
        unsafe fn call<R, F>(
            data: NonNull<Abstract>,
            ctx: NonNull<Abstract>,
            result: NonNull<Abstract>,
        ) where
            R: NodeValue,
            F: Fn(&mut SelectorContext<'_, R>) -> R + 'static,
        {
            let f = &*(data.cast::<F>().as_ref());
            let ctx = &mut *(ctx.cast::<SelectorContext<'_, R>>().as_ptr());

            let v = f(ctx);
            std::ptr::write(result.cast::<R>().as_ptr(), v)
        }

        // The function that drops the stored callback.
        unsafe fn drop<R, F>(data: NonNull<Abstract>)
        where
            R: NodeValue,
            F: Fn(&mut SelectorContext<'_, R>) -> R + 'static,
        {
            // Reconstruct and drop to free memory.
            let _ = Box::from_raw(data.cast::<F>().as_ptr());
        }

        // Box the callback and convert it to a raw pointer.
        let boxed_f = Box::new(f) as Box<F>;
        let raw_f = Box::into_raw(boxed_f);
        let data = unsafe { NonNull::new_unchecked(raw_f as *mut Abstract) };

        Computer {
            data,
            call: call::<R, F>,
            drop: drop::<R, F>,
            not_send: PhantomData::default(),
        }
    }

    /// Calls the stored callback with the provided context and returns the result  of type `V`.
    /// This function is `unsafe` because it assumes that `ctx` is of the correct type `V` and that
    /// the stored callback corresponds to this type.
    pub(super) unsafe fn compute<R: NodeValue>(&self, ctx: &mut SelectorContext<'_, R>) -> R {
        let mut result: R = std::mem::MaybeUninit::uninit().assume_init();
        let ctx_ptr = NonNull::from(ctx).cast::<Abstract>();
        let result_ptr = NonNull::new(&mut result as *mut _ as *mut Abstract).unwrap();

        (self.call)(self.data, ctx_ptr, result_ptr);
        result
    }
}

/// Represents the context in which a selector operates.
/// Holds a mutable reference to the main `Context` and a weak reference to the selector.
#[derive(Deref, DerefMut)]
pub struct Selector<V: NodeValue> {
    #[deref]
    #[deref_mut]
    pub(super) p_node: ProtoNode,
    result_typ: PhantomData<V>,
}

impl<T: NodeValue> Clone for Selector<T> {
    fn clone(&self) -> Self {
        Self {
            p_node: self.p_node.clone(),
            result_typ: self.result_typ,
        }
    }
}

impl<V: NodeValue> AnyNode<V> for Selector<V> {
    type Weak = WeakNode<V, Selector<V>>;

    fn key(&self) -> NodeKey {
        self.p_node.key
    }

    fn downgrade(&self) -> Self::Weak {
        WeakNode {
            wp_node: self.p_node.downgrade(),
            value_typ: self.result_typ,
            node_typ: PhantomData::<Selector<V>>,
        }
    }

    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Selector {
            p_node: weak.wp_node.upgrade()?,
            result_typ: weak.value_typ,
        })
    }
}

impl<V: NodeValue> Selector<V> {
    pub(super) fn new(key: NodeKey, rc: Weak<RwLock<NodeRefCounter>>) -> Self {
        Self {
            p_node: ProtoNode::new(key, rc),
            result_typ: PhantomData,
        }
    }

    pub fn read<'a>(&self, ctx: &'a mut Context) -> &'a V {
        ctx.read_selector(self)
    }
}

#[derive(Deref, DerefMut, Clone)]
pub(super) struct SelectorImMap(NodeImMap);

impl SelectorImMap {
    pub fn new() -> Self {
        Self(NodeImMap::new())
    }

    pub(super) fn lookup(&self, key: &NodeKey) -> bool {
        self.values.contains_key(key)
    }

    pub(super) fn remove(&mut self, key: &NodeKey) {
        self.values
            .remove(key)
            // Panic at this point most likely signals a bug in the program.
            // The reason why the key may not be in the map:
            // - The value has already been deleted
            // - The value is currently leased and is being updated
            .unwrap_or_else(|| panic!("cannot delete a node value that does not exist"));
    }

    pub(super) fn reserve<V>(
        &self,
        create_slot: impl FnOnce(&Self, NodeKey) -> Selector<V>,
    ) -> Slot<V, Selector<V>>
    where
        V: NodeValue,
    {
        let key = self.rc.write().counts.insert(1.into());
        Slot(create_slot(self, key), PhantomData)
    }

    pub(super) fn insert<V>(&mut self, key: NodeKey, value: V)
    where
        V: NodeValue,
    {
        self.values.insert(key, Box::new(value));
    }

    pub(super) fn read<V>(&self, key: &NodeKey) -> &V
    where
        V: NodeValue,
    {
        // TODO: add check for valid context

        self.values[key]
            .as_any_ref()
            .downcast_ref()
            .unwrap_or_else(|| {
                panic!(
                    "cannot read {} node that is being updated",
                    std::any::type_name::<V>()
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        context_v2::{node::AnyNodeValue, ContextCell},
        executor::{BackgroundExecutor, MainThreadExecutor},
        platform::{AnyDispatcher, AnyPlatform},
    };

    use super::*;
    use std::{
        any::Any,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
    };

    #[derive(Debug, Clone, PartialEq)]
    struct TestString(String);

    impl AnyNodeValue for TestValue {
        fn as_any_ref(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[derive(Debug, Clone)]
    struct TestValue {
        a: usize,
    }

    impl AnyNodeValue for TestString {
        fn as_any_ref(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    struct MockPlatform {}

    struct MockDispatcher {}

    impl AnyDispatcher for MockDispatcher {
        fn dispatch(&self, _runnable: async_task::Runnable) {
            todo!()
        }

        fn dispatch_on_main_thread(&self, _runnable: async_task::Runnable) {
            todo!()
        }

        fn park(&self, _timeout: Option<std::time::Duration>) -> bool {
            todo!()
        }

        fn unparker(&self) -> parking::Unparker {
            todo!()
        }
    }

    impl AnyPlatform for MockPlatform {
        fn main_thread_executor(&self) -> MainThreadExecutor {
            MainThreadExecutor::new(Arc::new(MockDispatcher {}))
        }

        fn background_executor(&self) -> BackgroundExecutor {
            BackgroundExecutor::new(Arc::new(MockDispatcher {}))
        }
    }

    #[test]
    fn test_computer_creation_and_compute() {
        let ctx_cell = &mut ContextCell::new(Rc::new(MockPlatform {}));
        let ctx: &mut Context = &mut *ctx_cell.borrow_mut();

        let atom_a = ctx.create_atom(|_| TestValue { a: 0 });

        ctx.update_atom(&atom_a, |this, _| {
            this.a += 10;
        });

        let atom_a_key = atom_a.key();
        let selector_a = ctx.create_selector(move |selector_context| {
            let atom_a_value = selector_context.read::<TestValue>(&atom_a_key);

            let result = format!("Hello, {}!", atom_a_value.a);
            TestString(result)
        });

        let selector_a_result = selector_a.read(ctx);
        assert_eq!(selector_a_result, &TestString("Hello, 10!".to_string()));

        ctx.update_atom(&atom_a, |this, atom_context| {
            this.a += 10;

            atom_context.notify();
        });

        let selector_a_result = selector_a.read(ctx);
        assert_eq!(selector_a_result, &TestString("Hello, 20!".to_string()));
    }

    #[test]
    fn test_computer_drop() {
        // A counter to track drop calls.
        struct DropCounter {
            count: Rc<AtomicUsize>,
        }

        impl Drop for DropCounter {
            fn drop(&mut self) {
                self.count.fetch_add(1, Ordering::SeqCst);
            }
        }

        let counter = Rc::new(AtomicUsize::new(0));
        let counter_clone = Rc::clone(&counter);

        let computer = {
            let drop_counter = DropCounter {
                count: counter_clone,
            };

            // Create a `Computer` that takes a DropCounter in its closure.
            Computer::new(move |_: &mut SelectorContext<'_, TestValue>| -> TestValue {
                // Closure uses drop_counter, which will be dropped when the Computer is dropped.
                let _ = &drop_counter;

                TestValue { a: 0 }
            })
        };

        assert_eq!(counter.load(Ordering::SeqCst), 0);
        drop(computer); //  must drop the DropCounter inside the closure
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
