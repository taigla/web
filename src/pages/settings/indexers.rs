#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::prelude::*;
use fermi::use_set;
use crate::components::modal::indexer::{Indexer, IndexerModalState, STATE};
use crate::services::settings::{Indexers, INDEXER_LIST_STORE, State, SettingCommand};

#[inline_props]
pub fn IndexerList<'a>(cx: Scope, indexers: &'a Indexers, on_indexer_select: EventHandler<'a, u64>) -> Element {
    let rows = indexers.iter().map(|indexer| {
        rsx! {
            tr {
                key: "{indexer.id}",
                td { "{indexer.name}" }
                td { "{indexer.priority}" }
                td {
                    button {
                        class: "btn sm",
                        onclick: move |_| on_indexer_select.call(indexer.id),
                        "Edit"
                    }
                }
            }
        }
    });

    render! {
        table {
            class: "table bordered",
            thead {
                tr {
                    th { class: "w-10/12", "Name" }
                    th { class: "w-1/12", "Priority" }
                    th { class: "w-1/12", "" }
                }
            }
            tbody {
                rows
            }
        }
    }
}

pub fn Indexers(cx: Scope) -> Element {
    let set_modal_status = use_set(cx, &STATE);
    let indexers = use_read(cx, &INDEXER_LIST_STORE);
    let setting_handle = use_coroutine_handle::<SettingCommand>(cx);

    use_memo(cx, (), |_| {
        setting_handle.map(|h| h.send(SettingCommand::FetchIndexerList));
        log::info!("Hello")
    });

    render! {
        div {
            class: "flex flex-col w-full",
            div {
                class: "flex flex-row justify-between pb-2",
                p { class: "text-2xl", "Indexers" }
                p {
                    onclick: move |_| set_modal_status(IndexerModalState::New),
                    class: "btn solid sm primary", "New"
                }
            }
            match &indexers {
                State::Ok(indexers) => rsx! {
                    IndexerList {
                        indexers: indexers,
                        on_indexer_select: move |id| set_modal_status(IndexerModalState::Id(id))
                    }
                },
                State::Loading => rsx! { "Loading" },
                _ => rsx! { "Error" }
            }
            Indexer {}
        }
    }
}
