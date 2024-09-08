#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_logger::tracing::Level;
use web_sys::window;
use taigla::routes::Routes;
use taigla::api::{TaiglaApi, Token};
// use taigla::services::use_init_service;
use taigla::redux::use_init_store;
use taigla::store::TaiglaStore;

fn api_address() -> String {
    match env!("TAIGLA_BACKEND_URL") {
        "" => {
            let window = window().unwrap();
            window.location().origin().unwrap()
        },
        u => u.to_string()
    }
}

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("Failed to init logger");
    launch(App);
}

fn App() -> Element {
    use_init_store(TaiglaStore::new);
    // use_init_atom_root(cx);
    // use_shared_state_provider(|| Token::default());
    // let token = use_shared_state::<Token>().unwrap();
    let api_address = api_address();
    // let api = TaiglaApi::new(&api_address, token.read().clone());
    // use_shared_state_provider(cx, || api.clone());
    // use_init_service(cx, api.clone());

    rsx! {
        div {
            class: "h-screen",
            Router::<Routes> {}
        }
    }
}
