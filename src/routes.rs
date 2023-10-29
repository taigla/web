use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::pages::Home;
use crate::pages::Settings;

#[derive(Clone, Routable)]
pub enum Routes {
    #[route("/")]
    #[redirect("/:.._segments", |_segments: Vec<String>| Routes::Home {})]
    Home {},
    #[route("/settings")]
    Settings {},
}
