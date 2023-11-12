#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use web_sys::window;
use crate::hooks::use_taigla_api;
use crate::routes::Routes;
use crate::api::{Token, LoginParam};

pub fn Login(cx: Scope) -> Element {
    let navigator = use_navigator(&cx);
    let token = use_shared_state::<Token>(cx).unwrap();
    let api = use_taigla_api(cx);

    render! {
        "Login"
        form {
            onsubmit: move |event| {
                cx.spawn({
                    to_owned![navigator, token, api];
                    async move {
                        log::info!("{:?}", event);
                        let username: &str = event.data.values.get("name").unwrap().get(0).unwrap();
                        let password: &str = event.data.values.get("password").unwrap().get(0).unwrap();
                        let response = api.read().post_login(&LoginParam { username, password })
                            .await;
                        if let Ok(response) = response {
                            let local_storage = window()
                                .unwrap()
                                .local_storage()
                                .unwrap()
                                .unwrap();
                            local_storage.set_item("token", &response.token).unwrap();
                            log::info!("{}", response.token);
                            token.write().set(&response.token);
                            navigator.replace(Routes::Home {});
                        }
                    }
                })
            },
            input { name: "name" },
            input { name: "password", r#type: "password" }
            input { r#type: "submit", value: "Login" }
        }
    }
}
