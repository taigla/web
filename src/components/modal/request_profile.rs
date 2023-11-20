#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::{use_read, use_set, Atom};
use serde_json::{json, Value};
use super::ModalWithTitle;
use crate::hooks::{use_taigla_api, use_query};
use crate::services::settings::SettingCommand;
use crate::api::{RequestProfileRow, RequestProfile, QueryState};

pub static STATE: Atom<RequestProfileModalState> = Atom(|_| RequestProfileModalState::Close);

#[derive(PartialEq)]
pub enum RequestProfileModalState {
    New,
    Id(u64),
    Close
}

#[inline_props]
fn Input<'a>(cx: Scope, name: &'a str, lbl: Option<&'a str>, default_value: Option<&'a str>) -> Element<'a> {
    render! {
        Fragment {
            if let Some(lbl) = *lbl {
                rsx! { label { class: "col-span-12 md:col-span-3", lbl } }
            }
            rsx! { input { class: "input col-span-12 md:col-span-9", initial_value: *default_value, name: *name } }
        }
    }
}

#[inline_props]
fn Select<'a>(cx: Scope, name: &'a str, lbl: Option<&'a str>, default_value: Option<&'a str>, options: Vec<&'a str>) -> Element<'a> {
    let selected = default_value.unwrap_or("");
    let child = options.iter().map(|opt| {
        rsx! {
            option {selected: *opt == selected , *opt }
        }
    });

    render! {
        Fragment {
            if let Some(lbl) = *lbl {
                rsx! { label { class: "col-span-12 md:col-span-3", lbl } }
            }
            rsx! {
                select {
                    class: "select col-span-12 md:col-span-9",
                    name: *name,
                    child
                }
            }
        }
    }
}

#[inline_props]
fn Form<'a>(cx: Scope, request_profile: Option<&'a RequestProfile>, on_update: EventHandler<'a, Value>) -> Element<'a> {
    let min_file_size = if let Some(request_profile) = cx.props.request_profile { request_profile.min_file_size.to_string() } else { "".to_string() };
    let max_file_size = if let Some(request_profile) = cx.props.request_profile { request_profile.max_file_size.to_string() } else { "".to_string() };
    let set_state = use_set(cx, &STATE);

    let submit = move |evt: Event<FormData>| {
        log::info!("{:?}", evt);
        let body = json!({
            "name": evt.data.values.get("name").unwrap().get(0).unwrap(),
            "language": evt.data.values.get("language").unwrap().get(0).unwrap(),
            "quality": evt.data.values.get("quality").unwrap().get(0).unwrap(),
            "min_file_size": evt.data.values.get("min_file_size").unwrap().get(0).unwrap().parse::<u64>().unwrap(),
            "max_file_size": evt.data.values.get("max_file_size").unwrap().get(0).unwrap().parse::<u64>().unwrap()
        });
        on_update.call(body);
    };

    render! {
        form {
            class: "grid grid-cols-12 gap-y-5 items-center",
            onsubmit: submit,
            Input {
                lbl: "Name:",
                name: "name",
                default_value: if let Some(request_profile) = request_profile { &request_profile.name } else { "" }
            }
            Select {
                lbl: "Language:",
                name: "language",
                default_value: if let Some(request_profile) = request_profile { &request_profile.language } else { "" },
                options: vec!["EN", "FR"]
            }
            Select {
                lbl: "Quality:",
                name: "quality",
                default_value: if let Some(request_profile) = request_profile { &request_profile.quality } else { "" },
                options: vec!["2160p", "1080p", "720p", "480p"]
            }
            Input {
                lbl: "Min file size:",
                name: "min_file_size",
                default_value: "{min_file_size}"
            }
            Input {
                lbl: "Max file size:",
                name: "max_file_size",
                default_value: "{max_file_size}"
            }
            div {
                class: "flex flex-row justify-end col-span-12 gap-2",
                p {
                    class: "btn solid md",
                    onclick: move |_| set_state(RequestProfileModalState::Close),
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
fn ModalEditRequestProfile<'a>(cx: Scope, id: &'a u64) -> Element {
    let api = use_taigla_api(&cx);
    let request_profile = use_query::<RequestProfile>(cx, &format!("/api/v1/request-profiles/{}", id));
    let set_state = use_set(cx, &STATE);
    let id = **id;
    let setting_handle = use_coroutine_handle::<SettingCommand>(cx).unwrap();

    let edit = move |v| {
        to_owned![api, id, set_state, setting_handle];
        cx.spawn(async move {
            let request_profile = api.read().patch_request_profile(id, v)
                .await;
            if let Ok(request_profile) = request_profile {
                set_state(RequestProfileModalState::Close);
                setting_handle.send(SettingCommand::UpdateRequestProfile(RequestProfileRow {
                    id: request_profile.id,
                    name: request_profile.name
                }));
            }
        });
    };

    render! {
        match &request_profile {
            QueryState::Ok(i) => rsx! { Form {
                request_profile: i,
                on_update: edit
            } },
            QueryState::Loading => rsx! { "Loading" },
            _ => rsx! { "Error" }
        }
    }
}

#[inline_props]
fn ModalNewRequestProfile(cx: Scope) -> Element {
    let set_state = use_set(cx, &STATE);
    let api = use_taigla_api(&cx);
    let setting_handle = use_coroutine_handle::<SettingCommand>(cx).unwrap();

    let create = move |v| {
        to_owned![api, set_state, setting_handle];
        cx.spawn(async move {
            let request_profile = api.read().post_request_profile(v)
                .await;
            if let Ok(request_profile) = request_profile {
                set_state(RequestProfileModalState::Close);
                setting_handle.send(SettingCommand::AddRequestProfile(RequestProfileRow {
                    id: request_profile.id,
                    name: request_profile.name
                }));
            }
        });
    };

    render! {
        Form {
            on_update: create
        }
    }
}

#[inline_props]
pub fn RequestProfile(cx: Scope) -> Element {
    let state = use_read(cx, &STATE);

    render! {
        ModalWithTitle {
            visible: *state != RequestProfileModalState::Close,
            title: "RequestProfile",
            match state {
                RequestProfileModalState::Id(id) => rsx! { ModalEditRequestProfile { id: id } },
                RequestProfileModalState::New => rsx! { ModalNewRequestProfile {} },
                _ => rsx! { "" }
            }
        }
    }
}
