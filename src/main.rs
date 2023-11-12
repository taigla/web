#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::use_init_atom_root;
use log::LevelFilter;
use taigla::routes::Routes;
use taigla::api::{TaiglaApi, Token};
use taigla::services::use_init_service;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("Failed to init logger");
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    use_shared_state_provider(cx, || Token::default());
    let token = use_shared_state::<Token>(cx).unwrap();
    let api = TaiglaApi::new("http://localhost:8000", token.read().clone());
    use_shared_state_provider(cx, || api.clone());
    use_init_service(cx, api.clone());

    cx.render(rsx! {
        div {
            class: "h-screen bg-bw-50 text-neutral-950 dark:text-neutral-50",
            Router::<Routes> {}
        }
    })
}
