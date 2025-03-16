pub trait Command<T> {
    fn execute(&mut self) -> T;
}