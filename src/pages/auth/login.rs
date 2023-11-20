#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Serialize, Deserialize};
use web_sys::window;
use crate::routes::Routes;

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

    render! {
        "Login"
        form {
            onsubmit: move |event| {
                cx.spawn({
                    let navigator = navigator.to_owned();
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
                        navigator.replace(Routes::Home {});
                        log::info!("{}", response.token);
                    }
                })
            },
            input { name: "name" },
            input { name: "password", r#type: "password" }
            input { r#type: "submit", value: "Login" }
        }
    }
}
