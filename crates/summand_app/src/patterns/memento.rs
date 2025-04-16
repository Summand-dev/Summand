pub trait Memento<State> {
    fn export_memento() -> State;
    fn import_memento(memento: State);
}

pub trait MementoCareTaker<Memento>{
    fn has_memento(memento: Memento) -> bool;
    fn forget_memento(index: i8) -> Option<Memento>;
    fn add_memento(memento: Memento);
    fn remove_memento(index: i8)-> Option<Memento>;
}