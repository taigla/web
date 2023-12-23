use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    rc::Rc
};
use dioxus::prelude::*;
use super::store::{ReduxStore, Store};
use super::subscription::Subscription;

pub fn use_slice<
    'a,
    F: Copy + 'static + Fn(&S) -> T,
    S: 'static + 'a + Store,
    T: 'static + Clone + PartialEq,
>(
    cx: Scope,
    slicer: F,
) -> &ReduxSlice<T> {
    let store = cx.consume_context::<ReduxStore<S>>().unwrap();
    let subscribe = cx.use_hook({
        to_owned![store];

        move || {
            let gen_value_getter = {
                to_owned![store];
                move || {
                    let store = &store.store.borrow();
                    slicer(store)
                }
            };

            store.subscribe(cx.scope_id(), TypeId::of::<F>(), gen_value_getter, || {
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
            })
        }
    });

    cx.use_hook(|| ReduxSlice {
        subscribe: Rc::new(subscribe.clone()),
        _phantom: PhantomData,
    })
}

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

fn downcast<T: Any>(v: Rc<RefCell<Box<dyn Any>>>) -> Rc<RefCell<Box<T>>> {
    let v: *const RefCell<Box<dyn Any>> = Rc::into_raw(v);
    unsafe { Rc::from_raw(v as *const RefCell<Box<T>>) }
}
