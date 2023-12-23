use std::{
    any::TypeId,
    collections::HashMap,
    rc::Rc
};
use dioxus::prelude::{ScopeId, RefCell};
use super::value::ValueEntry;

pub(super) type Subscriptions = Rc<RefCell<HashMap<TypeId, ValueEntry>>>;

#[derive(Clone)]
pub(super) struct Subscription {
    pub value_entry: ValueEntry,
    pub subscriptions: Subscriptions,
    pub function_id: TypeId,
    pub scope_id: ScopeId,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        let mut subscriptions = self.subscriptions.borrow_mut();

        let no_more_subscriptions = {
            let function = subscriptions.get_mut(&self.function_id);
            if let Some(function) = function {
                // Unsubscribe this scope
                function.scopes.borrow_mut().remove(&self.scope_id);
                function.scopes.borrow().is_empty()
            } else {
                false
            }
        };

        if no_more_subscriptions {
            // Remove the subscription itself if there are no more subscribers
            subscriptions.remove(&self.function_id);
        }
    }
}