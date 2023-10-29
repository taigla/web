#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use taigla::routes::Routes;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router::<Routes> {}
    })
}
