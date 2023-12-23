use dioxus::prelude::Scope;
use super::store::{Store, ReduxStore};

#[derive(Clone)]
pub struct ReduxDispatcher<S: Store> {
    // Dispatch events
    event_dispatcher: async_channel::Sender<S::Event>,
}

impl<S: Store> ReduxDispatcher<S> {
    pub fn dispatch(&self, event: S::Event) {
        // TODO: Handle errors
        self.event_dispatcher.try_send(event).unwrap();
    }
}

pub fn use_dispatcher<S: 'static + Store>(cx: Scope) -> ReduxDispatcher<S> {
    let store = cx.consume_context::<ReduxStore<S>>().unwrap();
    ReduxDispatcher {
        event_dispatcher: store.event_dispatcher,
    }
}
