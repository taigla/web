use dioxus::prelude::*;
use std::rc::Rc;
use fermi::prelude::*;
use futures_util::stream::StreamExt;
use crate::api::{TaiglaApi, IndexerRow, UserRow, Job, WorkerState, QueryState, Invite, RequestProfileRow};

pub enum SettingCommand {
    FetchIndexerList,
    UpdateIndexer(IndexerRow),
    AddIndexer(IndexerRow),
    FetchUserList,
    FetchWorkersList,
    FetchCronjobsList,
    FetchInvitesList,
    FetchRequestProfilesList,
    UpdateRequestProfile(RequestProfileRow),
    AddRequestProfile(RequestProfileRow)
}

pub type IndexerStore = QueryState<Vec<IndexerRow>>;
pub static INDEXER_LIST_STORE: Atom<IndexerStore> = Atom(|_| QueryState::NotFetch);

pub type UserStore = QueryState<Vec<UserRow>>;
pub static USER_LIST_STORE: Atom<UserStore> = Atom(|_| QueryState::NotFetch);

pub type WorkerStore = QueryState<Vec<WorkerState>>;
pub static WORKER_LIST_STORE: Atom<WorkerStore> = Atom(|_| QueryState::NotFetch);

pub type CronjobStore = QueryState<Vec<Job>>;
pub static CRONJOB_LIST_STORE: Atom<CronjobStore> = Atom(|_| QueryState::NotFetch);

pub type InviteStore = QueryState<Vec<Invite>>;
pub static INVITE_LIST_STORE: Atom<InviteStore> = Atom(|_| QueryState::NotFetch);

pub type RequestProfileStore = QueryState<Vec<RequestProfileRow>>;
pub static REQUEST_PROFILE_LIST_STORE: Atom<RequestProfileStore> = Atom(|_| QueryState::NotFetch);

pub async fn settings_service(mut rx: UnboundedReceiver<SettingCommand>, api: TaiglaApi, atoms: Rc<AtomRoot>) {
    while let Some(msg) = rx.next().await {
        match msg {
            SettingCommand::FetchIndexerList => {
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
                let users = api.get_users().await;
                let new_value = match users {
                    Ok(k) => QueryState::Ok(k),
                    Err(e) => QueryState::Err(e)
                };
                atoms.set((&USER_LIST_STORE).unique_id(), new_value);
            },
            SettingCommand::FetchWorkersList => {
                let workers = api.get_workers().await;
                let new_value = match workers {
                    Ok(k) => QueryState::Ok(k),
                    Err(e) => QueryState::Err(e)
                };
                atoms.set((&WORKER_LIST_STORE).unique_id(), new_value);
            },
            SettingCommand::FetchCronjobsList => {
                let cronjobs = api.get_cronjobs().await;
                let new_value = match cronjobs {
                    Ok(k) => QueryState::Ok(k),
                    Err(e) => QueryState::Err(e)
                };
                atoms.set((&CRONJOB_LIST_STORE).unique_id(), new_value);
            },
            SettingCommand::FetchInvitesList => {
                let invites = api.get_invites().await;
                let new_value = match invites {
                    Ok(k) => QueryState::Ok(k),
                    Err(e) => QueryState::Err(e)
                };
                atoms.set((&INVITE_LIST_STORE).unique_id(), new_value);
            },
            SettingCommand::FetchRequestProfilesList => {
                let request_profiles = api.get_request_profiles().await;
                let new_value = match request_profiles {
                    Ok(k) => QueryState::Ok(k),
                    Err(e) => QueryState::Err(e)
                };
                atoms.set((&REQUEST_PROFILE_LIST_STORE).unique_id(), new_value);
            },
            SettingCommand::UpdateRequestProfile(request_profile) => {
                let current = (*atoms.read(&REQUEST_PROFILE_LIST_STORE)).clone();
                match current {
                    QueryState::Ok(mut c) => {
                        let i = c.iter_mut().find(|e| e.id == request_profile.id);
                        if let Some(index) = i {
                            *index = request_profile;
                            atoms.set((&REQUEST_PROFILE_LIST_STORE).unique_id(), QueryState::Ok(c));
                        }
                    },
                    _ => ()
                };
            },
            SettingCommand::AddRequestProfile(request_profile) => {
                let current = (*atoms.read(&REQUEST_PROFILE_LIST_STORE)).clone();
                match current {
                    QueryState::Ok(mut c) => {
                        c.push(request_profile);
                        atoms.set((&REQUEST_PROFILE_LIST_STORE).unique_id(), QueryState::Ok(c));
                    },
                    _ => ()
                };
            }
        }
    }
}