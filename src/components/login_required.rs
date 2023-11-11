#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::routes::Routes;
use crate::api::Token;

pub fn LoginRequired(cx: Scope) -> Element {
    let navigator = use_navigator(cx);
    let token = use_shared_state::<Token>(cx).unwrap();

    if token.read().is_empty() {
        navigator.replace(Routes::Login {});
        return render! {""}
    }
    render! {
        Outlet::<Routes> {}
    }
}
