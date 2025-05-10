use summand_app::summand::Summand;
use uuid::Uuid;

pub trait DataAdapter<T> {
    fn new() -> T;
    fn create(&self, summand: &Summand);
    fn update(&self, id: Uuid, summand: &Summand);
    fn remove(&self, id: Uuid);
    fn find(&self, id: Uuid) -> Option<Summand>;
    fn find_all(&self) -> Vec<Summand>;
}
