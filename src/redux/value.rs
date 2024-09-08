use std::{
    any::Any,
    collections::HashSet,
    rc::Rc,
    cell::RefCell
};
use dioxus::prelude::ScopeId;

pub(super) type ValueComparer = Rc<dyn Fn(&Rc<RefCell<Box<dyn Any>>>) -> bool>;

#[derive(Clone)]
pub(super) struct ValueEntry {
    // Scopes subscribed to this value
    pub scopes: Rc<RefCell<HashSet<ScopeId>>>,
    // The actual value
    pub value: Rc<RefCell<Box<dyn Any>>>,
    // A function to compare the cached and new value
    pub compare: ValueComparer,
}