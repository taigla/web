#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::redux::{use_dispatcher, use_slice};
use crate::store::TaiglaStore;
use crate::reducers::auth::AuthAction;
// use crate::reducers::{use_get_version, ApiEvent, RequestState, TaiglaStore};
use std::any::TypeId;

#[component]
pub fn Home() -> Element {
    let dispatcher = use_dispatcher::<TaiglaStore>();
    // let version = use_get_version();
    let token = use_slice(|state: &TaiglaStore| state.auth.token.clone());
    let token = use_slice(|state: &TaiglaStore| state.auth.token2.clone());
    let mut counter = use_signal(|| 0);

    // tracing::info!("{:?}", version);
    let onclick = move |_| {
        dispatcher.dispatch(AuthAction::SetToken(format!("Hello {counter}")));
        counter += 1;
    };
    // // let tmp = version();
    // match version.read().clone() {
    //     RequestState::Ok(e) => (),
    //     _ => ()
    // };
    rsx! {
        "Home"
        "{counter}"
        "{token}"
        button {
            class: "btn",
            onclick: onclick,
            "New Task"
        }
    }
}
