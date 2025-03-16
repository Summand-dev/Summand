pub trait Observable<Observer> {
    fn register_observer(&mut self, listener: Observer);
    fn unregister_observer(&mut self, listener: Observer);
}