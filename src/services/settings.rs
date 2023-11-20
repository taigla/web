use dioxus::prelude::*;
use serde::Deserialize;
use crate::states::TaiglaApi;
use std::rc::Rc;
use fermi::prelude::*;
use futures_util::stream::StreamExt;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct IndexerRow {
    pub id: u64,
    pub name: String,
    pub priority: u8
}

pub type Indexers = Vec<IndexerRow>;

#[derive(Clone, PartialEq)]
pub enum State<T> {
    NotFetch,
    Loading,
    Ok(T)
}

pub enum SettingCommand {
    FetchIndexerList,
    UpdateIndexer(IndexerRow),
    AddIndexer(IndexerRow)
}

type IndexerStore = State<Indexers>;

pub static INDEXER_LIST_STORE: Atom<IndexerStore> = Atom(|_| {
    IndexerStore::NotFetch
});

pub async fn settings_service(mut rx: UnboundedReceiver<SettingCommand>, api: TaiglaApi, atoms: Rc<AtomRoot>) {
    while let Some(msg) = rx.next().await {
        match msg {
            SettingCommand::FetchIndexerList => {
                if *atoms.read(&INDEXER_LIST_STORE) != State::NotFetch {
                    return;
                }
                let indexers = api.get("/api/v1/indexers")
                    .send()
                    .await
                    .unwrap()
                    .json::<Indexers>()
                    .await
                    .unwrap();
                let new_value = State::Ok(indexers);
                atoms.set((&INDEXER_LIST_STORE).unique_id(), new_value);
            },
            SettingCommand::UpdateIndexer(indexer) => {
                let current = (*atoms.read(&INDEXER_LIST_STORE)).clone();
                match current {
                    State::Ok(mut c) => {
                        let i = c.iter_mut().find(|e| e.id == indexer.id);
                        if let Some(index) = i {
                            *index = indexer;
                            atoms.set((&INDEXER_LIST_STORE).unique_id(), State::Ok(c));
                        }
                    },
                    _ => ()
                };
            },
            SettingCommand::AddIndexer(indexer) => {
                let current = (*atoms.read(&INDEXER_LIST_STORE)).clone();
                match current {
                    State::Ok(mut c) => {
                        c.push(indexer);
                        atoms.set((&INDEXER_LIST_STORE).unique_id(), State::Ok(c));
                    },
                    _ => ()
                };
            }
        }
    }
}