#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::Deserialize;
use crate::hooks::use_swr::{use_swr, State};

#[derive(Deserialize, PartialEq)]
struct User {
    name: String,
    id: u64,
    disable: bool
}

#[inline_props]
fn UserList<'a>(cx: Scope, users: &'a Vec<User>) -> Element {
    let row = users.iter().map(|user| {
        rsx! {
            tr {
                key: "{user.id}",
                td { "{user.id}" }
                td { "{user.name}" }
                td { "{user.disable}" }
            }
        }
    });

    render! {
        table {
            thead {
                tr {
                    td { class: "w-1/12", "ID" }
                    td { class: "w-10/12", "Name" }
                    td { class: "w-1/12", "Disabled" }
                }
            }
            tbody {
                row
            }
        }
    }
}

pub fn Users(cx: Scope) -> Element {
    let users = use_swr(&cx, "/api/v1/users");

    render! {
        div {
            class: "flex flex-col w-full",
            "Users"
            match users {
                State::Ok(users) => rsx! { UserList { users: users } },
                _ => rsx! { "Loading" }
            }
        }
    }
}
