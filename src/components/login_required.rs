#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use web_sys::window;
use crate::routes::Routes;
use crate::states::TaiglaApi;

pub fn LoginRequired(cx: Scope) -> Element {
    let navigator = use_navigator(cx);
    let window = window().unwrap();
    let token = window.local_storage().unwrap().unwrap().get("token").unwrap();

    if token == None {
        navigator.replace(Routes::Login {});
        return render! {""}
    }
    let token = token.unwrap();
    use_shared_state_provider::<TaiglaApi>(cx, || TaiglaApi::new("http://localhost:8000", &token));

    render! {
        Outlet::<Routes> {}
    }
}
