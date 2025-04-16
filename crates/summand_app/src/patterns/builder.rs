pub trait Builder<Buildable> {
    fn build(self) -> Buildable;
}