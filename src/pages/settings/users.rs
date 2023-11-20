#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::prelude::*;
use crate::api::UserRow;
use crate::services::settings::{SettingCommand, USER_LIST_STORE, QueryState};

#[inline_props]
fn UserList<'a>(cx: Scope, users: &'a Vec<UserRow>) -> Element {
    let rows = users.iter().map(|user| {
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
            class: "table bordered",
            thead {
                tr {
                    th { class: "w-1/12", "ID" }
                    th { class: "w-10/12", "Name" }
                    th { class: "w-1/12", "Disabled" }
                }
            }
            tbody {
                rows
            }
        }
    }
}

pub fn Users(cx: Scope) -> Element {
    let setting_handle = use_coroutine_handle::<SettingCommand>(cx);
    let users = use_read(cx, &USER_LIST_STORE);

    use_memo(cx, (), |_| {
        setting_handle.map(|h| h.send(SettingCommand::FetchUserList));
    });

    render! {
        div {
            class: "flex flex-col w-full",
            p { class: "text-2xl", "Users" }
            match &users {
                QueryState::Ok(users) => rsx! { UserList { users: users } },
                QueryState::Loading => rsx! { "Loading" },
                _ => rsx! { "Error" }
            }
        }
    }
}
