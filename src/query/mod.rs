use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::any::Any;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub type QueryTagsExtractor<T: PartialEq> = Box<dyn Fn(&Box<dyn Any>) -> Vec<T>>;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Query {
    url: String,
    method: String,
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

struct QueryListeners<T: PartialEq> {
    value: QueryValue<Box<dyn Any>>,
    listeners: HashSet<ScopeId>,
    tags_extractor: QueryTagsExtractor<T>
}

#[derive(Clone)]
struct QueryProvider<T: PartialEq> {
    tags: Vec<T>,
    pub(super) queries_registry: Rc<RefCell<HashMap<Query, QueryListeners<T>>>>
}

impl<T: PartialEq> QueryProvider<T> {
    pub fn new() -> Self {
        Self {
            tags: vec![],
            queries_registry: Rc::new(RefCell::new(HashMap::default()))
        }
    }

    pub(super) fn add_listener(&self, query: Query, tag_extractor: QueryTagsExtractor<T>) -> QueryValue<Box<dyn Any>> {
        let mut registry = self.queries_registry.try_borrow_mut().unwrap();
        let listeners = registry
            .entry(query.clone())
            .or_insert(QueryListeners {
                value: Arc::new(RwLock::new(QueryResult::Fetching)),
                listeners: HashSet::new(),
                tags_extractor: tag_extractor
            });
        listeners.listeners.insert(current_scope_id().unwrap());
        listeners.value.clone()
    }

    pub(super) fn mutate(&self, query: &Query, value: QueryResult<Box<dyn Any>>) {
        let registry = self.queries_registry.borrow();
        let entry = registry.get(query);
        if let Some(entry) = entry {
            let storage = entry.value.try_write();
            if let Ok(mut storage) = storage {
                *storage = value;
                for scope_id in &entry.listeners {
                    schedule_update_any()(*scope_id);
                }
            } else {
                tracing::error!("Failed to lock query for writing");
            }
        } else {
            tracing::error!("Tying to mutate an unexisting query");
        }
    }
}

pub fn use_init_query_provider<T>()
where
    T: PartialEq + Clone + 'static
{
    use_context_provider(|| QueryProvider::<T>::new());
}

pub fn use_query<T, U, E>(query: Query, tag_extractor: E) -> QueryValue<Box<T>>
where
    T: DeserializeOwned + std::fmt::Debug + 'static,
    U: PartialEq + Clone + 'static,
    E: Fn(&Box<T>) -> Vec<U> + 'static
{
    let query_provider = use_context::<QueryProvider<U>>();

    use_hook(move || {
        let tag_extractor_wrapper = Box::new(move |result: &Box<dyn Any>| {
            let user: &Box<T> = result.downcast_ref().unwrap();
            tag_extractor(user)
        });

        let listener = query_provider.add_listener(query.clone(), tag_extractor_wrapper);

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

        let v: *const RwLock<QueryResult<Box<dyn Any>>> = Arc::into_raw(listener);
        unsafe { Arc::from_raw(v as *const RwLock<QueryResult<Box<T>>>) }
    })
}

// USed to know when to invalidate query du to mutation
#[derive(PartialEq, Clone)]
pub enum QueryTags {
    User(i32)
}

#[derive(Debug, Deserialize)]
pub struct User {}

pub fn use_get_user(user_id: i32) -> QueryValue<Box<User>> {
    let query = use_query(
        Query { url: "http://localhost:1234/api/v1/users".to_string(), ..Default::default() },
        Box::new(move |user: &Box<User>| {
            tracing::info!("{}", user_id);
            vec![QueryTags::User(0)]
        })
    );
    // let query = use_query::<User>(Query { url: "https://app.gama.ovh/api".to_string(), ..Default::default() });
    query
}

fn use_user_login() {
    // Return a function that you can call to send the request
}

fn tmp() {

}
