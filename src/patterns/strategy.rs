pub trait Strategy<T> {
    fn change_strategy(&self, from: T, to: T);
}