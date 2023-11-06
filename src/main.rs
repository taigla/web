#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::use_init_atom_root;
use log::LevelFilter;
use taigla::hooks::use_init_query_provider;
use taigla::routes::Routes;
use taigla::states::TaiglaApi;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("Failed to init logger");
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let api = TaiglaApi::new("http://localhost:8000", "ok");
    use_init_query_provider(cx, api);

    cx.render(rsx! {
        div {
            class: "h-screen bg-bw-50 text-neutral-950 dark:text-neutral-50",
            Router::<Routes> {}
        }
    })
}
