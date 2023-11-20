#![allow(non_snake_case)]
use dioxus::prelude::*;
use std::future::Future;
use std::rc::Rc;
use std::pin::Pin;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde_json::Value;

#[derive(Clone, Debug)]
pub(super) enum QueryValue {
    Loading,
    Ok(Value),
    Validating(Value),
    NotFetch,
    Error,
}

#[derive(Clone, Debug)]
pub(super) struct Entry {
    pub listeners: Vec<ScopeId>,
    pub value: QueryValue,
    // To avoid deserializing value at each rerender we store the "hash" of the latest deserialize value
    // along with the deserialized value.
    // At each render we compare the hash with the one store in the registry, if they are different we update
    // the deserialized value
    pub hash: ValueHash
}

pub trait Fetcher {
    fn get(&self, url: &str) -> Pin<Box<dyn Future<Output = Value>>>;
}

pub(super) type RegistryEntry = Arc<RwLock<Entry>>;
pub(super) type ValueHash = u32;

#[derive(Clone)]
pub struct UseQueryProvider {
    fetcher: Rc<dyn Fetcher>,
    registry: Rc<RefCell<HashMap<String, RegistryEntry>>>,
    scheduler: Arc<dyn Fn(ScopeId)>
}

impl UseQueryProvider {
    pub(super) fn add_listener<'a>(&'a self, url: &str, scope: ScopeId) -> RegistryEntry {
        self.registry
            .borrow_mut()
            .entry(url.to_string())
            .or_insert(Arc::new(RwLock::new(Entry {
                listeners: vec![scope],
                value: QueryValue::NotFetch,
                hash: 0
            })))
            .clone()
    }

    pub(super) fn remove_listener(&self, entry: &RegistryEntry, scope: ScopeId) {
        let mut writable_entry = entry.write().unwrap();

        writable_entry.listeners.retain(|e| e != &scope);
    }

    pub async fn update(&self, url: &str) {
        let registry = self.registry.borrow_mut();
        let entry = registry.get(url).unwrap();

        let response = self.fetcher.get(url).await;
        self.mutate(entry, response);
    }

    pub(super) fn mutate(&self, entry: &RegistryEntry, new_value: Value) {
        self.mutate_silent(entry, new_value);

        for listener in entry.read().unwrap().listeners.iter() {
            (self.scheduler)(listener.clone());
        }
    }

    pub(super) fn mutate_silent(&self, entry: &RegistryEntry, new_value: Value) {
        let mut writable_entry = entry.write().unwrap();
        writable_entry.value = QueryValue::Ok(new_value);
        writable_entry.hash += 1;
        drop(writable_entry);
    }
}

pub fn use_init_query_provider<T: Fetcher + 'static>(cx: &ScopeState, fetcher: T) -> UseQueryProvider {
    if let Some(provider) = cx.consume_context::<UseQueryProvider>() {
        provider
    } else {
        cx.provide_root_context(
            UseQueryProvider {
                fetcher: Rc::new(fetcher),
                registry: Rc::new(RefCell::new(HashMap::new())),
                scheduler: cx.schedule_update_any()
            }
        )
    }
}

pub fn use_query_provider(cx: &ScopeState) -> UseQueryProvider {
    if let Some(provider) = cx.consume_context::<UseQueryProvider>() {
        provider
    } else {
        panic!("Query provider not initialise");
    }
}
