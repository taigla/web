pub trait Reducer<T> {
    fn reduce(self, slice: &mut T);
}
