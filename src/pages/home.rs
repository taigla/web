#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::redux::use_dispatcher;
use crate::reducers::{TaiglaStore, use_get_version, ApiEvent};

#[component]
pub fn Home(cx: Scope) -> Element {
    let dispatcher = use_dispatcher::<TaiglaStore>(cx);
    let version = use_get_version(cx);
    let mut counter = use_state(cx, || 0);

    log::info!("{:?}", version);
    let onclick = move |_| {
        dispatcher.dispatch(ApiEvent::GetVersion);
        counter += 1;
    };
    render! {
        "Home"
        "{counter}"
        button {
            onclick: onclick,
            "New Task"
        }
    }
}
