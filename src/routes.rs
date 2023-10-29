use dioxus_router::prelude::*;
use dioxus::prelude::*;
use crate::pages::Home;

#[derive(Clone, Routable)]
pub enum Routes {
    #[route("/")]
    Home {}
}

