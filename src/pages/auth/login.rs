#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Serialize, Deserialize};
use web_sys::window;
use crate::routes::Routes;
use crate::api::Token;

#[derive(Serialize)]
struct BodyLogin<'a> {
    username: &'a str,
    password: &'a str
}

#[derive(Deserialize)]
struct ResponseLogin {
    token: String
}

pub fn Login(cx: Scope) -> Element {
    let navigator = use_navigator(&cx);
    let token = use_shared_state::<Token>(cx).unwrap();

    render! {
        "Login"
        form {
            onsubmit: move |event| {
                cx.spawn({
                    to_owned!(navigator, token);
                    async move {
                        log::info!("{:?}", event);
                        let username: &str = event.data.values.get("name").unwrap().get(0).unwrap();
                        let password: &str = event.data.values.get("password").unwrap().get(0).unwrap();
                        let response = reqwest::Client::new()
                            .post("http://localhost:8000/api/v1/auth/login")
                            .json(&BodyLogin { username, password })
                            .send()
                            .await
                            .unwrap()
                            .json::<ResponseLogin>()
                            .await
                            .unwrap();
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
                })
            },
            input { name: "name" },
            input { name: "password", r#type: "password" }
            input { r#type: "submit", value: "Login" }
        }
    }
}
