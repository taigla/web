#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;
use taigla::routes::Routes;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("Failed to init logger");
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router::<Routes> {}
    })
}
