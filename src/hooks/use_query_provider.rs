#![allow(non_snake_case)]
use dioxus::prelude::*;
use std::future::Future;
use std::rc::Rc;
use std::pin::Pin;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde_json::Value;

#[derive(Clone)]
pub(super) struct Entry {
    listeners: Vec<ScopeId>,
    pub value: Value
}

pub trait Fetcher {
    fn get(&self, url: &str) -> Pin<Box<dyn Future<Output = Value>>>;
}

type RegistryEntry = Arc<RwLock<Entry>>;

#[derive(Clone)]
pub struct UseQueryProvider {
    fetcher: Rc<dyn Fetcher>,
    registry: Rc<RefCell<HashMap<String, RegistryEntry>>>,
    scheduler: Arc<dyn Fn(ScopeId)>
}

impl UseQueryProvider {
    pub(super) fn add_listener<'a>(&'a self, url: &str, scope: ScopeId) -> Value {
        self.registry
            .borrow_mut()
            .entry(url.to_string())
            .or_insert(Arc::new(RwLock::new(Entry {
                listeners: vec![scope],
                value: Value::Null
            })))
            .clone()
            .read()
            .unwrap()
            .value
            .clone()
    }

    pub async fn update(&self, url: &str) {
        let registry = self.registry.borrow_mut();
        let entry = registry.get(url).unwrap();

        let response = self.fetcher.get(url).await;
        entry.write().unwrap().value = response;

        for listener in entry.read().unwrap().listeners.iter() {
            (self.scheduler)(listener.clone());
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
