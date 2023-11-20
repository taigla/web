#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use std::rc::Rc;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde_json::Value;

#[derive(Clone)]
struct Entry {
    listeners: Vec<ScopeId>,
    data: Value
}

pub trait Fetcher {
    fn get(&self, url: &str) -> Value;
}

type RegistryEntry = Arc<RwLock<Entry>>;

#[derive(Clone)]
pub struct UseQueryProvider {
    // fetcher: Rc<dyn Fetcher>,
    registry: Rc<RefCell<HashMap<String, RegistryEntry>>>,
    update: Arc<dyn Fn(ScopeId)>
}

impl UseQueryProvider {
    pub fn add_listener(&self, url: &str, scope: ScopeId) {
        let entry = self.registry
            .borrow_mut()
            .entry(url.to_string())
            .or_insert(Arc::new(RwLock::new(Entry {
                listeners: vec![scope],
                data: Value::Null
            })));
    }
}

pub fn use_init_query_provider(cx: &ScopeState) -> UseQueryProvider {
    if let Some(provider) = cx.consume_context::<UseQueryProvider>() {
        provider
    } else {
        cx.provide_root_context(
            UseQueryProvider {
                // fetcher: Rc::new(fetcher),
                registry: Rc::new(RefCell::new(HashMap::new())),
                update: cx.schedule_update_any()
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
