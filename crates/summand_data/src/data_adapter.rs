use summand_app::summand::Summand;
use uuid::Uuid;

pub trait DataAdapter<T> {
    async fn new() -> T;
    async fn create(&self, summand: &Summand);
    async fn update(&self, id: Uuid, summand: &Summand);
    async fn remove(&self, id: Uuid);
    async fn find(&self, id: Uuid) -> Option<Summand>;
    async fn find_all(&self) -> Vec<Summand>;
}
