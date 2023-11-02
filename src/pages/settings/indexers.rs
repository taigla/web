#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::Deserialize;
use crate::{hooks::use_swr::{use_swr, State}, routes::Routes};
use crate::components::modal::Indexer;

#[derive(Deserialize)]
pub struct Indexer {
    id: u64,
    name: String,
    priority: u8
}

#[inline_props]
pub fn IndexerList<'a>(cx: Scope, indexers: &'a Vec<Indexer>) -> Element {
    let rows = indexers.iter().map(|indexer| {
        rsx! {
            tr {
                key: "{indexer.id}",
                td { "{indexer.name}" }
                td { "{indexer.priority}" }
            }
        }
    });

    render! {
        table {
            class: "table",
            thead {
                tr {
                    td { class: "w-11/12", "Name" }
                    td { class: "w-1/12", "Priority" }
                }
            }
            tbody {
                rows
            }
        }
    }
}

pub fn Indexers(cx: Scope) -> Element {
    let indexers = use_swr::<Vec<Indexer>>(&cx, "/api/v1/indexers");

    render! {
        div {
            class: "flex flex-col w-full px-24",
            div {
                class: "flex flex-row justify-between pb-2",
                p { class: "text-2xl", "Indexers" }
                Link { to: Routes::Home {}, class: "btn solid sm primary", "New" }
            }
            match indexers {
                State::Ok(indexers) => rsx! { IndexerList { indexers: indexers } },
                _ => rsx! { "Loading" }
            }
            Indexer { id: "12".to_string() }
        }
    }
}
