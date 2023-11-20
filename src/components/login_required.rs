#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::routes::Routes;

enum State {

}

pub fn LoginRequired(cx: Scope) -> Element {
    render! {
        Outlet::<Routes> {}
    }
}
