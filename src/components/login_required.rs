#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::reducers::TaiglaStore;
use crate::redux::use_slice;
use crate::routes::Routes;

pub fn LoginRequired(cx: Scope) -> Element {
    let navigator = use_navigator(cx);
    let token = use_slice(cx, TaiglaStore::token);

    if token.read().borrow().is_empty() {
        navigator.replace(Routes::Login {});
        return render! {""}
    }
    render! {
        Outlet::<Routes> {}
    }
}
