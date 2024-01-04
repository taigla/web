use super::store::Store;
use super::effect::Effect;

pub trait Reducer<T: Store> {
    fn reduce(self, store: &mut T) -> Effect<T>;
}
