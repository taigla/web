#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::Deserialize;
use crate::hooks::use_taigla_api;

#[derive(Deserialize, PartialEq)]
struct User {
    name: String,
    id: u64,
    disable: bool
}

enum State<T> {
    Loading,
    Ok(T),
    Err,
}

fn use_users(cx: Scope) -> &UseState<State<Vec<User>>> {
    let users = use_state::<State<Vec<User>>>(cx, || State::Loading);
    let taigla_api = use_taigla_api(cx);

    use_effect(cx, (), |_| {
        to_owned![taigla_api, users];
        async move {
            let response = taigla_api.read().get("/api/v1/users")
                .send()
                .await
                .unwrap()
                .json::<Vec<User>>()
                .await
                .expect("Failed to parse user list");
            users.set(State::Ok(response));
        }
    });

    users
}

#[inline_props]
fn UserList<'a>(cx: Scope, users: &'a Vec<User>) -> Element {
    let row = users.iter().map(|user| {
        rsx! {
            p {
                key: "{user.id}",
                "{user.name}"
            }
        }
    });

    render! { row }
}

pub fn Users(cx: Scope) -> Element {
    let users = use_users(cx);

    render! {
        "Users"
        match users.get() {
            State::Ok(users) => rsx! { UserList { users: users } },
            _ => rsx! { "Loading" }
        }
    }
}
