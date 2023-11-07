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
    Error,
}

#[derive(Clone, Debug)]
pub(super) struct Entry {
    listeners: Vec<ScopeId>,
    pub value: QueryValue,
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
                value: QueryValue::Loading,
                hash: 0
            })))
            .clone()
    }

    pub async fn update(&self, url: &str) {
        let registry = self.registry.borrow_mut();
        let entry = registry.get(url).unwrap();

        let response = self.fetcher.get(url).await;
        let mut writable_entry = entry.write().unwrap();
        writable_entry.value = QueryValue::Ok(response);
        writable_entry.hash += 1;
        drop(writable_entry);

        for listener in entry.read().unwrap().listeners.iter() {
            (self.scheduler)(listener.clone());
            log::info!("Send event");
        }
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
