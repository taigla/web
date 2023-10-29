use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::pages::{Home, Settings};
use crate::components::Header;

#[derive(Clone, Routable)]
pub enum Routes {
    #[layout(Header)]
        #[route("/")]
        #[redirect("/:.._segments", |_segments: Vec<String>| Routes::Home {})]
        Home {},
        #[route("/settings")]
        Settings {},
    // #[end_layout]
}
