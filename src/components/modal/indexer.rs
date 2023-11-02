#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde_json::json;
use serde::Deserialize;
use super::Modal;
use crate::hooks::use_taigla_api;

#[derive(Deserialize)]
struct Indexer {
    id: u64,
    name: String,
    url: String,
    api_key: String,
    priority: u8
}

#[inline_props]
fn Input<'a>(cx: Scope, name: &'a str, lbl: Option<&'a str>) -> Element {
    render! {
        Fragment {
            if let Some(lbl) = lbl {
                rsx! { label { class: "col-span-12 md:col-span-3", "{lbl}" } }
            }
            rsx! { input { class: "input col-span-12 md:col-span-9", name: *name } }
        }
    }
}

#[inline_props]
pub fn Form<'a>(cx: Scope, on_close: EventHandler<'a, ()>) -> Element<'a> {
    let api = use_taigla_api(&cx);

    let submit = move |evt: Event<FormData>| {
        to_owned![api];
        cx.spawn(async move {
            log::info!("{:?}", evt);
            let body = json!({
                "name": evt.data.values.get("name").unwrap().get(0).unwrap(),
                "url": evt.data.values.get("url").unwrap().get(0).unwrap(),
                "api_key": evt.data.values.get("api_key").unwrap().get(0).unwrap(),
                "priority": evt.data.values.get("priority").unwrap().get(0).unwrap().parse::<u8>().unwrap()
            });
            let response = api.read().post("/api/v1/indexers")
                .json(&body)
                .send()
                .await
                .unwrap()
                .json::<Indexer>()
                .await
                .unwrap();
        })
    };

    render! {
        form {
            class: "grid grid-cols-12 gap-y-5 items-center",
            onsubmit: submit,
            Input { lbl: "Name:", name: "name" }
            Input { lbl: "Url:", name: "url" }
            Input { lbl: "Api key:", name: "api_key" }
            Input { lbl: "Priority:", name: "priority" }
            div {
                class: "flex flex-row justify-end col-span-12 gap-2",
                p {
                    class: "btn solid md",
                    onclick: move |_| on_close.call(()),
                    "Close"
                }
                input {
                    class: "btn solid primary md",
                    r#type: "submit",
                    value: "Save"
                }
            }
        }
    }
}

#[inline_props]
pub fn Indexer<'a>(cx: Scope, id: String, visible: &'a bool, on_close: EventHandler<'a, ()>) -> Element<'a> {
    render! {
        Modal {
            visible: **visible,
            div {
                class: "flex flex-col p-6",
                p { class: "text-2xl", "Indexer" }
                Form {
                    on_close: move |_| on_close.call(())
                }
            }
        }
    }
}
