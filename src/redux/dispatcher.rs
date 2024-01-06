use dioxus::core::ScopeState;
use super::store::{Store, ReduxStore};

pub struct ReduxDispatcher<S: Store> {
    // Dispatch events
    pub(super) event_dispatcher: async_channel::Sender<S::Event>,
}

impl<S: Store> ReduxDispatcher<S> {
    pub fn dispatch(&self, event: S::Event) {
        // TODO: Handle errors
        self.event_dispatcher.try_send(event).unwrap();
    }
}

pub fn use_dispatcher<S: 'static + Store>(cx: &ScopeState) -> ReduxDispatcher<S> {
    let store = cx.consume_context::<ReduxStore<S>>().unwrap();
    ReduxDispatcher {
        event_dispatcher: store.event_dispatcher,
    }
}

impl<T: Store> Clone for ReduxDispatcher<T> {
    fn clone(&self) -> Self {
        Self {
            event_dispatcher: self.event_dispatcher.clone()
        }
    }
}
