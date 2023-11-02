#![allow(non_snake_case)]
use dioxus::prelude::*;
use reqwest;

pub use indexers::Indexers;
pub use invites::Invites;
pub use request_profiles::RequestProfiles;
pub use users::Users;

mod indexers;
mod invites;
mod request_profiles;
mod users;

pub fn Settings(cx: Scope) -> Element {
    log::info!("Hello");

    use_future(cx, (), |_| async move {
        let response = reqwest::get("http://localhost:8000/api/v1/version")
            .await
            .unwrap()
            .text()
            .await;
        log::info!("{:?}", response)
    });

    render! {
        "settings"
    }
}
