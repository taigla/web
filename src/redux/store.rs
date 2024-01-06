use std::{
    collections::HashSet,
    rc::Rc,
    sync::Arc,
    pin::Pin,
    future::Future
};
use dioxus::prelude::*;
use super::subscription::Subscriptions;
use super::value::{ValueComparer, ValueEntry};
use super::subscription::Subscription;
use super::simple_hash::SimpleHash;
use super::effect::{Effect, InnerEffect};
use super::dispatcher::ReduxDispatcher;

const REQUEST_WORKER_COUNT: u8 = 2;

pub trait Store: Sized {
    type Event;

    fn handle(&mut self, event: Self::Event) -> Effect<Self>;
}

pub struct ReduxStore<S: Store> {
    // Actual provided store
    pub(super) store: Rc<RefCell<S>>,
    // Dispatch events
    pub(super) event_dispatcher: async_channel::Sender<S::Event>,
    // Subscribers
    subscriptions: Subscriptions,

    schedule_update_any: Arc<dyn Fn(ScopeId)>,
    requests: async_channel::Sender<Pin<Box<dyn Future<Output = ()>>>>
}

impl<S: Store> ReduxStore<S> {
    fn handle(&self, event: S::Event) {
        // Notify the store of the new event
        let effect = self.store.borrow_mut().handle(event);

        match effect.0 {
            InnerEffect::None => (),
            InnerEffect::Future(f) => {
                let dispatcher = ReduxDispatcher { event_dispatcher: self.event_dispatcher.clone() };
                let future = f(dispatcher);
                self.requests.send_blocking(future).unwrap();
                return;
            }
        }

        for (_function, value_entry) in self.subscriptions.borrow().iter() {
            let cached_value = &value_entry.value;
            let is_equal = (value_entry.compare)(cached_value);
            if !is_equal {
                // Because the cached and new values were not the same this marks as dirty all the scopes subscribed to those values
                for scope_id in value_entry.scopes.borrow().iter() {
                    (self.schedule_update_any)(*scope_id)
                }
            }
        }
    }

    pub(super) fn subscribe<V: 'static>(
        &self,
        scope_id: ScopeId,
        function_id: SimpleHash,
        value: impl FnOnce() -> V,
        compare: impl FnOnce() -> ValueComparer,
    ) -> Subscription {
        let value_entry = {
            let mut subscriptions = self.subscriptions.borrow_mut();
            subscriptions
                .entry(function_id)
                .and_modify(|entry| {
                    entry.scopes.borrow_mut().insert(scope_id);
                })
                .or_insert_with(|| ValueEntry {
                    scopes: Rc::new(RefCell::new(HashSet::from([scope_id]))),
                    value: Rc::new(RefCell::new(Box::new(value()))),
                    compare: compare(),
                })
                .clone()
        };

        Subscription {
            value_entry,
            subscriptions: self.subscriptions.clone(),
            function_id,
            scope_id,
        }
    }
}

impl<S: Store> Clone for ReduxStore<S> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            event_dispatcher: self.event_dispatcher.clone(),
            subscriptions: self.subscriptions.clone(),
            schedule_update_any: self.schedule_update_any.clone(),
            requests: self.requests.clone()
        }
    }
}

pub fn use_init_store<S: Store + 'static>(cx: Scope, create_store: impl FnOnce() -> S) {
    cx.use_hook(|| {
        let (event_tx, event_rx) = async_channel::unbounded::<S::Event>();
        let (request_notif_tx, request_notif_rx) = async_channel::unbounded();

        let store = cx.provide_context(ReduxStore {
            store: Rc::new(RefCell::new(create_store())),
            event_dispatcher: event_tx,
            subscriptions: Rc::default(),
            schedule_update_any: cx.schedule_update_any(),
            requests: request_notif_tx
        });

        for _ in 0..REQUEST_WORKER_COUNT {
            let request_notif_rx = request_notif_rx.clone();
            cx.spawn(async move {
                while let Ok(f) = request_notif_rx.recv().await {
                    f.await;
                }
            });
        }

        cx.spawn(async move {
            while let Ok(event) = event_rx.recv().await {
                store.handle(event)
            }
        });
    });
}

