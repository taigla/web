use dioxus::prelude::*;
use super::store::{Store, ReduxStore};

pub struct ReduxDispatcher<S: Store> {
    // Dispatch events
    pub(super) event_dispatcher: async_channel::Sender<S::Event>,
}

impl<S: Store> ReduxDispatcher<S> {
    pub fn dispatch<T: Into<S::Event>>(&self, event: T) {
        // TODO: Handle errors
        self.event_dispatcher.try_send(event.into()).unwrap();
    }
}

pub fn use_dispatcher<S: 'static + Store>() -> ReduxDispatcher<S> {
    let store = consume_context::<ReduxStore<S>>();

    use_hook(|| {
        ReduxDispatcher {
            event_dispatcher: store.event_dispatcher,
        }
    })
}

impl<T: Store> Clone for ReduxDispatcher<T> {
    fn clone(&self) -> Self {
        Self {
            event_dispatcher: self.event_dispatcher.clone()
        }
    }
}
