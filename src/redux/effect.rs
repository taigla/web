use std::future::Future;
use std::pin::Pin;
use super::dispatcher::ReduxDispatcher;
use super::store::Store;

pub(super) enum InnerEffect<T: Store> {
    None,
    Future(Box<dyn FnOnce(ReduxDispatcher<T>) -> Pin<Box<dyn Future<Output = ()>>>>)
}

pub struct Effect<T: Store>(pub(super) InnerEffect<T>);

impl<T: Store> Effect<T> {
    pub const NONE: Self = Self(InnerEffect::None);

    pub fn future(future: impl FnOnce(ReduxDispatcher<T>) -> Pin<Box<dyn Future<Output = ()>>> + 'static) -> Self {
        Self(InnerEffect::Future(Box::new(future)))
    }
}
