#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use web_sys::window;
use crate::hooks::use_taigla_api;
use crate::reducers::{TaiglaStore, TaiglaEvent};
use crate::redux::use_dispatcher;
use crate::routes::Routes;
use crate::api::{Token, LoginParam};
use crate::icons::TaiglaLogo;

pub fn Login(cx: Scope) -> Element {
    let navigator = use_navigator(&cx);
    let token = use_shared_state::<Token>(cx).unwrap();
    let api = use_taigla_api(cx);
    let error = use_state::<Option<String>>(cx, || None);
    let loading = use_state::<bool>(cx, || false);
    let dispatcher = use_dispatcher::<TaiglaStore>(cx);

    let on_submit = move |event: Event<FormData>| {
        cx.spawn({
            to_owned![navigator, token, api, error, loading, dispatcher];
            async move {
                tracing::info!("{:?}", event);
                loading.set(true);
                let username: &str = event.data.values.get("name").unwrap().get(0).unwrap();
                let password: &str = event.data.values.get("password").unwrap().get(0).unwrap();
                let response = api.read().post_login(&LoginParam { username, password })
                    .await;
                match response {
                    Ok(response) => {
                        let local_storage = window()
                            .unwrap()
                            .local_storage()
                            .unwrap()
                            .unwrap();
                        local_storage.set_item("token", &response.token).unwrap();
                        tracing::info!("{}", response.token);
                        token.write().set(&response.token);
                        dispatcher.dispatch(TaiglaEvent::SetToken(response.token));
                        navigator.replace(Routes::Home {});
                    },
                    Err(e) => { error.set(Some(e.err_code)) }
                }
                loading.set(false);
            }
        })
    };

    render! {
        div {
            class: "modal modal-open absolute top-0 left-0 bottom-0 right-0",
            div {
                class: "modal-backdrop"
            }
            div {
                class: "modal-box max-w-none w-[32rem]",
                form {
                    class: "flex flex-col",
                    onsubmit: on_submit,
                    div {
                        class: "flex flex-row items-center justify-center mb-10 mt-2",
                        TaiglaLogo { width: 75, height: 75 }
                        p { class:"ml-4 font-semibold text-4xl", "Taigla" }
                    }
                    label { "Name:" }
                    input { class: "input input-bordered", name: "name" },
                    label { class: "mt-4", "Password:" }
                    input { class: "input input-bordered", name: "password", r#type: "password" }
                    if *error != None { rsx! { p { class: "text-error mt-4", "Invalid user/password" } } } else { rsx! { "" } }
                    button {
                        class: "btn btn-primary mt-4",
                        disabled: **loading,
                        if **loading { rsx! { span { class: "loading loading-spinner" } } } else { rsx! { "Login" } }
                    }
                }
            }
        }
    }
}
