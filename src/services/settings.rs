use dioxus::prelude::*;
use std::rc::Rc;
use fermi::prelude::*;
use futures_util::stream::StreamExt;
use crate::api::{TaiglaApi, IndexerRow, UserRow, ApiError};

#[derive(Clone)]
pub enum QueryState<T> {
    NotFetch,
    Loading,
    Ok(T),
    Err(ApiError)
}

pub enum SettingCommand {
    FetchIndexerList,
    UpdateIndexer(IndexerRow),
    AddIndexer(IndexerRow),
    FetchUserList
}

pub type IndexerStore = QueryState<Vec<IndexerRow>>;
pub static INDEXER_LIST_STORE: Atom<IndexerStore> = Atom(|_| QueryState::NotFetch);

pub type UserStore = QueryState<Vec<UserRow>>;
pub static USER_LIST_STORE: Atom<UserStore> = Atom(|_| QueryState::NotFetch);

pub async fn settings_service(mut rx: UnboundedReceiver<SettingCommand>, api: TaiglaApi, atoms: Rc<AtomRoot>) {
    while let Some(msg) = rx.next().await {
        match msg {
            SettingCommand::FetchIndexerList => {
                if !matches!(*atoms.read(&INDEXER_LIST_STORE), QueryState::NotFetch) {
                    return;
                }
                let indexers = api.get_indexers().await;
                let new_value = match indexers {
                    Ok(k) => QueryState::Ok(k),
                    Err(e) => QueryState::Err(e)
                };
                atoms.set((&INDEXER_LIST_STORE).unique_id(), new_value);
            },
            SettingCommand::UpdateIndexer(indexer) => {
                let current = (*atoms.read(&INDEXER_LIST_STORE)).clone();
                match current {
                    QueryState::Ok(mut c) => {
                        let i = c.iter_mut().find(|e| e.id == indexer.id);
                        if let Some(index) = i {
                            *index = indexer;
                            atoms.set((&INDEXER_LIST_STORE).unique_id(), QueryState::Ok(c));
                        }
                    },
                    _ => ()
                };
            },
            SettingCommand::AddIndexer(indexer) => {
                let current = (*atoms.read(&INDEXER_LIST_STORE)).clone();
                match current {
                    QueryState::Ok(mut c) => {
                        c.push(indexer);
                        atoms.set((&INDEXER_LIST_STORE).unique_id(), QueryState::Ok(c));
                    },
                    _ => ()
                };
            },
            SettingCommand::FetchUserList => {
                if !matches!(*atoms.read(&USER_LIST_STORE), QueryState::NotFetch) {
                    return;
                }
                let users = api.get_users().await;
                let new_value = match users {
                    Ok(k) => QueryState::Ok(k),
                    Err(e) => QueryState::Err(e)
                };
                atoms.set((&USER_LIST_STORE).unique_id(), new_value);
            }
        }
    }
}