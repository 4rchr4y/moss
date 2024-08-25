use moss_std::collection::{FxHashSet, VecDeque};
use std::any::{Any, TypeId};

use super::{
    entity::{Entity, EntityId, EntityMap, Slot},
    model::{Model, ModelContext},
    subscriber::{SubscriberSet, Subscription},
};

pub struct Reservation<T>(pub(crate) Slot<T>);

pub trait EventEmitter<E: Any>: 'static {}

pub trait Context {
    type Result<T>;

    fn reserve_model<T: 'static>(&mut self) -> Self::Result<Reservation<T>>;

    fn new_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>>;

    fn insert_model<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>>;

    fn update_model<T, R>(
        &mut self,
        handle: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> Self::Result<R>
    where
        T: 'static;
}

pub enum Effect {
    Notify {
        emitter: EntityId,
    },
    Emit {
        emitter: EntityId,
        event_type: TypeId,
        event: Box<dyn Any>,
    },
    Defer {
        callback: Box<dyn FnOnce(&mut PlatformContext) + 'static>,
    },
}

type Handler = Box<dyn FnMut(&mut PlatformContext) -> bool + 'static>;
type Listener = Box<dyn FnMut(&dyn Any, &mut PlatformContext) -> bool + 'static>;
type ReleaseListener = Box<dyn FnOnce(&mut dyn Any, &mut PlatformContext) + 'static>;

pub struct PlatformContext {
    pub(crate) observers: SubscriberSet<EntityId, Handler>,
    pub(crate) pending_notifications: FxHashSet<EntityId>,
    pub(crate) pending_effects: VecDeque<Effect>,
    pending_updates: usize,
    pub(crate) entities: EntityMap,
    flushing_effects: bool,
    pub(crate) event_listeners: SubscriberSet<EntityId, (TypeId, Listener)>,
    pub(crate) release_listeners: SubscriberSet<EntityId, ReleaseListener>,
}

impl Context for PlatformContext {
    type Result<T> = T;

    fn reserve_model<T: 'static>(&mut self) -> Self::Result<Reservation<T>> {
        Reservation(self.entities.reserve())
    }

    fn new_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Model<T> {
        self.update(|ctx| {
            let slot = ctx.entities.reserve();

            let entity = build_model(&mut ModelContext::new(ctx, slot.downgrade()));
            ctx.entities.insert(slot, entity)
        })
    }

    fn insert_model<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Model<T> {
        self.update(|ctx| {
            let slot = reservation.0;
            let entity = build_model(&mut ModelContext::new(ctx, slot.downgrade()));
            ctx.entities.insert(slot, entity)
        })
    }

    fn update_model<T: 'static, R>(
        &mut self,
        model: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> R
    where
        T: 'static,
    {
        self.update(|ctx| {
            let mut entity = ctx.entities.lease(model);
            let result = update(&mut entity, &mut ModelContext::new(ctx, model.downgrade()));
            ctx.entities.end_lease(entity);
            result
        })
    }
}

impl PlatformContext {
    pub fn new() -> Self {
        Self {
            observers: SubscriberSet::new(),
            pending_notifications: FxHashSet::default(),
            pending_effects: VecDeque::new(),
            pending_updates: 0,
            entities: EntityMap::new(),
            flushing_effects: false,
            event_listeners: SubscriberSet::new(),
            release_listeners: SubscriberSet::new(),
        }
    }

    pub fn defer(&mut self, f: impl FnOnce(&mut PlatformContext) + 'static) {
        self.push_effect(Effect::Defer {
            callback: Box::new(f),
        })
    }

    pub fn push_effect(&mut self, effect: Effect) {
        match &effect {
            Effect::Notify { emitter } => {
                if !self.pending_notifications.insert(*emitter) {
                    return;
                }
            }
            _ => {}
        };

        self.pending_effects.push_back(effect);
    }

    pub fn subscribe<T, E, Event>(
        &mut self,
        entity: &E,
        mut on_event: impl FnMut(E, &Event, &mut PlatformContext) + 'static,
    ) -> Subscription
    where
        T: 'static + EventEmitter<Event>,
        E: Entity<T>,
        Event: 'static,
    {
        self.subscribe_internal(entity, move |entity, event, ctx| {
            on_event(entity, event, ctx);
            true
        })
    }

    pub(crate) fn new_subscription(
        &mut self,
        key: EntityId,
        value: (TypeId, Listener),
    ) -> Subscription {
        let (subscription, activate) = self.event_listeners.insert(key, value);
        self.defer(move |_| activate());
        subscription
    }

    pub(crate) fn subscribe_internal<T, E, Evt>(
        &mut self,
        entity: &E,
        mut on_event: impl FnMut(E, &Evt, &mut PlatformContext) -> bool + 'static,
    ) -> Subscription
    where
        T: 'static + EventEmitter<Evt>,
        E: Entity<T>,
        Evt: 'static,
    {
        let entity_id = entity.entity_id();
        let entity = entity.downgrade();

        self.new_subscription(
            entity_id,
            (
                TypeId::of::<Evt>(),
                Box::new(move |event, cx| {
                    let event: &Evt = event.downcast_ref().expect("invalid event type");
                    if let Some(handle) = E::upgrade_from(&entity) {
                        on_event(handle, event, cx)
                    } else {
                        false
                    }
                }),
            ),
        )
    }

    pub(crate) fn new_observer(&mut self, key: EntityId, value: Handler) -> Subscription {
        let (subscription, activate) = self.observers.insert(key, value);
        self.defer(move |_| activate());

        subscription
    }

    pub fn observe<W, E>(
        &mut self,
        entity: &E,
        mut on_notify: impl FnMut(E, &mut PlatformContext) -> bool + 'static,
    ) -> Subscription
    where
        W: 'static,
        E: Entity<W>,
    {
        self.observe_internal(entity, move |e, ctx| {
            on_notify(e, ctx);
            true
        })
    }

    pub(crate) fn observe_internal<W, E>(
        &mut self,
        entity: &E,
        mut on_notify: impl FnMut(E, &mut PlatformContext) -> bool + 'static,
    ) -> Subscription
    where
        W: 'static,
        E: Entity<W>,
    {
        let entity_id = entity.entity_id();
        let handle = entity.downgrade();
        self.new_observer(
            entity_id,
            Box::new(move |ctx| {
                if let Some(handle) = E::upgrade_from(&handle) {
                    on_notify(handle, ctx)
                } else {
                    false
                }
            }),
        )
    }

    pub(crate) fn update<R>(&mut self, update: impl FnOnce(&mut PlatformContext) -> R) -> R {
        self.pending_updates += 1;
        let result = update(self);
        if !self.flushing_effects && self.pending_updates == 1 {
            self.flushing_effects = true;
            self.flush_effects();
            self.flushing_effects = false;
        }

        self.pending_updates -= 1;
        result
    }

    fn flush_effects(&mut self) {
        loop {
            self.release_dropped_entities();

            if let Some(effect) = self.pending_effects.pop_front() {
                match effect {
                    Effect::Notify { emitter } => {
                        self.apply_notify_effect(emitter);
                    }
                    Effect::Emit {
                        emitter,
                        event_type,
                        event,
                    } => self.apply_emit_effect(emitter, event_type, event),
                    Effect::Defer { callback } => {
                        self.apply_defer_effect(callback);
                    }
                }
            } else {
                if self.pending_effects.is_empty() {
                    break;
                }
            }
        }
    }

    fn release_dropped_entities(&mut self) {
        loop {
            let dropped = self.entities.take_dropped();
            if dropped.is_empty() {
                break;
            }

            for (entity_id, mut entity) in dropped {
                self.observers.remove(&entity_id);
                self.event_listeners.remove(&entity_id);
                for release_callback in self.release_listeners.remove(&entity_id) {
                    release_callback(entity.as_mut(), self);
                }
            }
        }
    }

    fn apply_notify_effect(&mut self, emitter: EntityId) {
        self.pending_notifications.remove(&emitter);

        self.observers
            .clone()
            .retain(&emitter, |handler| handler(self));
    }

    fn apply_defer_effect(&mut self, callback: Box<dyn FnOnce(&mut PlatformContext) + 'static>) {
        callback(self);
    }

    fn apply_emit_effect(&mut self, emitter: EntityId, event_type: TypeId, event: Box<dyn Any>) {
        self.event_listeners
            .clone()
            .retain(&emitter, |(stored_type, handler)| {
                if *stored_type == event_type {
                    handler(event.as_ref(), self)
                } else {
                    true
                }
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Counter {
        count: usize,
    }

    struct Change {
        increment: usize,
    }

    impl EventEmitter<Change> for Counter {}

    #[test]
    fn test_notification() {
        let mut ctx = PlatformContext::new();
        let counter: Model<Counter> = ctx.new_model(|_cx| Counter { count: 0 });

        let subscription = ctx.observe(&counter, |counter, ctx| {
            dbg!("Counter was notified! Current count");
            true
        });

        counter.update(&mut ctx, |counter, ctx| {
            counter.count += 1;
            ctx.notify();
        });

        subscription.detach()
    }

    #[test]
    fn test_subscription() {
        let mut ctx = PlatformContext::new();
        let counter: Model<Counter> = ctx.new_model(|_cx| Counter { count: 0 });
        let subscriber = ctx.new_model(|cx: &mut ModelContext<Counter>| {
            cx.subscribe(&counter, |subscriber, _emitter, event, _cx| {
                subscriber.count += event.increment * 2;
            })
            .detach();

            Counter {
                count: counter.read(cx).count * 2,
            }
        });

        counter.update(&mut ctx, |counter, cx| {
            counter.count += 2;
            cx.notify();
            cx.emit(Change { increment: 2 });
        });

        // assert_eq!(subscriber.read(&mut cx).count, 4);
        println!("{}", subscriber.read(&mut ctx).count);
    }
}
