#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::use_init_atom_root;
use log::LevelFilter;
use web_sys::window;
use taigla::routes::Routes;
use taigla::api::{TaiglaApi, Token};
use taigla::services::use_init_service;

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
    dioxus_logger::init(LevelFilter::Info).expect("Failed to init logger");
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    use_shared_state_provider(cx, || Token::default());
    let token = use_shared_state::<Token>(cx).unwrap();
    let api_address = api_address();
    log::info!("Using api: {}", api_address);
    let api = TaiglaApi::new(&api_address, token.read().clone());
    use_shared_state_provider(cx, || api.clone());
    use_init_service(cx, api.clone());

    cx.render(rsx! {
        div {
            class: "h-screen",
            Router::<Routes> {}
        }
    })
}
