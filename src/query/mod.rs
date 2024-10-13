use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::any::Any;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use serde::Deserialize;


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Query {
    url: String,
    method: String
}

impl Default for Query {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            method: "GET".to_string()
        }
    }
}

#[derive(Debug)]
pub enum QueryResult<T: Sized + std::fmt::Debug> {
    Ok(T),
    Fetching,
    Err
}

pub type QueryValue<T> = Arc<RwLock<QueryResult<T>>>;

#[derive(Debug)]
struct QueryListeners {
    value: QueryValue<Box<dyn Any>>,
    listeners: HashSet<ScopeId>
}

#[derive(Clone)]
struct QueryProvider {
    pub(super) queries_registry: Rc<RefCell<HashMap<Query, QueryListeners>>>
}

impl QueryProvider {
    pub fn new() -> Self {
        Self {
            queries_registry: Rc::new(RefCell::new(HashMap::default()))
        }
    }

    pub(super) fn add_listener(&self, query: Query) -> QueryValue<Box<dyn Any>> {
        let mut registry = self.queries_registry.try_borrow_mut().unwrap();
        let listeners = registry
            .entry(query.clone())
            .or_insert(QueryListeners {
                value: Arc::new(RwLock::new(QueryResult::Fetching)),
                listeners: HashSet::new()
            });
        listeners.listeners.insert(current_scope_id().unwrap());
        listeners.value.clone()
    }

    pub(super) fn mutate(&self, query: &Query, value: QueryResult<Box<dyn Any>>) {
        let registry = self.queries_registry.borrow();
        let entry = registry.get(query);
        if let Some(entry) = entry {
            let mut storage = entry.value.write().unwrap();
            *storage = value;
            for scope_id in &entry.listeners {
                schedule_update_any()(*scope_id);
            }
        }
    }
}

pub fn use_init_query_provider() {
    use_context_provider(|| QueryProvider::new());
}

pub fn use_query<T: DeserializeOwned>(query: Query) -> QueryValue<Box<dyn Any>> {
    let query_provider = use_context::<QueryProvider>();

    use_hook(move || {
        let listener = query_provider.add_listener(query.clone());

        spawn({
            to_owned![query];
            async move {
                let result = reqwest::get(&query.url).await;
                match result {
                    Ok(r) => {
                        match r.json::<T>().await {
                            Ok(response) => {
                                tracing::info!("Got response");
                            },
                            Err(e) => {
                                tracing::error!("Failed to deserialize {:?}", e);
                                query_provider.mutate(&query, QueryResult::Err);
                            }
                        }
                    }
                    Err(e) => tracing::error!("Request failed {:?}", e)
                }
            }
        });

        listener
    })
}

// USed to know when to invalidate query du to mutation
pub enum QueryTags {

}

#[derive(Debug, Deserialize)]
pub struct User {}

pub fn use_get_user(user_id: i32) -> QueryValue<User> {
    let query = use_query::<User>(Query { url: "http://localhost:1234/api/v1/users".to_string(), ..Default::default() });
    // let query = use_query::<User>(Query { url: "https://app.gama.ovh/api".to_string(), ..Default::default() });
    let tmp: Box<dyn Any> = Box::new(String::from("oki"));
    let aa: Box<String> = tmp.downcast().unwrap();

    let zzzz = query.read().unwrap();
    let v: *const RwLock<QueryResult<Box<dyn Any>>> = Arc::into_raw(query.clone());
    unsafe { Arc::from_raw(v as *const RwLock<QueryResult<User>>) }
}

fn use_user_login() {
    // Return a function that you can call to send the request
}

fn tmp() {

}
