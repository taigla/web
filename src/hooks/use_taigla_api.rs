use dioxus::prelude::*;
use crate::states::TaiglaApi;

pub fn use_taigla_api(cx: &ScopeState) -> &UseSharedState<TaiglaApi> {
    use_shared_state::<TaiglaApi>(cx).expect("Taigla api not provided")
}
