use std::{
    any::Any,
    marker::PhantomData,
    rc::Rc,
    cell::RefCell,
    any::TypeId,
    fmt::Display
};
use dioxus::prelude::*;
use super::store::{ReduxStore, Store};
use super::subscription::Subscription;

pub fn use_slice<
    F: 'static + Fn(&S) -> T,
    S: 'static + Store,
    T: 'static + Clone + PartialEq
>(
    slicer: F,
) -> ReduxSlice<T> {
    let store = consume_context::<ReduxStore<S>>();
    let subscribe = use_hook({
        to_owned![store];
        move || {
            let value = slicer(&store.store.borrow());
            tracing::info!("{:?}", TypeId::of::<T>());
            Rc::new(store.subscribe(current_scope_id().unwrap(), TypeId::of::<T>(), value, || {
                to_owned![store];
                Rc::new(move |cached: &Rc<RefCell<Box<dyn Any>>>| {
                    let store = &store.store.borrow();
                    let current = slicer(store);

                    // Compare cached and the new value
                    let is_equal = {
                        let cached = cached.borrow();
                        let cached = cached.downcast_ref::<T>().unwrap();
                        cached == &current
                    };

                    if !is_equal {
                        // Update the cached value with the new one
                        *cached.borrow_mut() = Box::new(current);
                    }
                    is_equal
                })
            }))
        }
    });

    use_hook(|| ReduxSlice {
        subscribe: subscribe,
        _phantom: PhantomData,
    })
}

#[derive(Clone)]
pub struct ReduxSlice<T> {
    subscribe: Rc<Subscription>,
    _phantom: PhantomData<T>,
}

impl<T: 'static> ReduxSlice<T> {
    pub fn read(&self) -> Rc<RefCell<Box<T>>> {
        let value = self.subscribe.value_entry.value.clone();
        downcast(value)
    }
}

impl<T: Display + 'static> Display for ReduxSlice<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.read().borrow().as_ref().fmt(f)
    }
}

fn downcast<T: Any>(v: Rc<RefCell<Box<dyn Any>>>) -> Rc<RefCell<Box<T>>> {
    let v: *const RefCell<Box<dyn Any>> = Rc::into_raw(v);
    unsafe { Rc::from_raw(v as *const RefCell<Box<T>>) }
}
