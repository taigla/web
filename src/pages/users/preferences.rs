#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::ui::Input;
use crate::hooks::use_taigla_api;

pub fn ChangePassword(cx: Scope) -> Element {
    let api = use_taigla_api(cx);

    let on_password_change = move |event: Event<FormData>| {
        cx.spawn({
            to_owned![api];
            async move {
                log::info!("{:?}", event);
                let password: &str = event.data.values.get("new_password").unwrap().get(0).unwrap();
                let _ = api.read().post_change_password(password)
                    .await;
            }
        })
    };
    render! {
        form {
            onsubmit: on_password_change,
            Input {
                lbl: "New password:",
                name: "new_password"
            }
            input {
                class: "btn btn-primary",
                r#type: "submit",
                value: "Save"
            }
        }
    }
}

pub fn UserPreferences(cx: Scope) -> Element {
    render! {
        div {
            ChangePassword {}
        }
    }
}
