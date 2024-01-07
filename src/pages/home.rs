#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::redux::use_dispatcher;
use crate::reducers::{TaiglaStore, use_get_version, ApiEvent};

#[component]
pub fn Home(cx: Scope) -> Element {
    let dispatcher = use_dispatcher::<TaiglaStore>(cx);
    let version = use_get_version(cx);

    log::info!("{:?}", version);
    let onclick = move |_| dispatcher.dispatch(ApiEvent::GetVersion);
    render! {
        "Home"
        button {
            onclick: onclick,
            "New Task"
        }
    }
}
