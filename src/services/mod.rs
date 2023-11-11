use dioxus::prelude::{ScopeState, use_coroutine};
use fermi::use_atom_root;
use crate::states::TaiglaApi;

pub mod settings;

pub fn use_init_service(cx: &ScopeState, api: TaiglaApi) {
    let atom_root = use_atom_root(cx);
    use_coroutine(cx, |rx| settings::settings_service(rx, api.clone(), atom_root.clone()));
}
