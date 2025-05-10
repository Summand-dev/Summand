use duckdb::{params, Connection};
use summand_app::summand::{self, Summand};
use uuid::Uuid;

use crate::data_adapter::DataAdapter;

#[derive(Debug)]
pub struct DockdbDataAdapter {
    client: Connection,
}

impl Clone for DockdbDataAdapter {
    fn clone(&self) -> Self {
        DockdbDataAdapter {
            client: self.client.try_clone().unwrap(),
        }
    }
}

struct SummandDoc {
    data: String,
}

impl DataAdapter<DockdbDataAdapter> for DockdbDataAdapter {
    fn new() -> DockdbDataAdapter {
        let connection = Connection::open("./summand.db3");
        if connection.is_ok() {
            return DockdbDataAdapter {
                client: connection.unwrap(),
            };
        } else {
            return DockdbDataAdapter {
                client: Connection::open_in_memory().unwrap(),
            };
        }
    }

    fn create(&self, summand: &summand_app::summand::Summand) {
        let summand_json = serde_json::to_string(summand);
        let _ = self.client.execute(
            "INSERT INTO summand (id, data) VALUES (?, ?)",
            params![summand.id.to_string(), summand_json.unwrap()],
        );
    }

    fn update(&self, id: uuid::Uuid, summand: &summand_app::summand::Summand) {
        let summand_json = serde_json::to_string(summand);
        let _ = self.client.execute(
            "INSERT INTO person (id, data) VALUES (?, ?)",
            params![summand.id.to_string(), summand_json.unwrap()],
        );
    }

    fn remove(&self, id: uuid::Uuid) {
        todo!()
    }

    fn find(&self, id: uuid::Uuid) -> Option<Summand> {
        todo!()
    }

    fn find_all(&self) -> Vec<Summand> {
        let summands_query = self.client.prepare("SELECT id,data FROM summand");
        if summands_query.is_ok() {
            let summands: Vec<Summand> = summands_query
                .unwrap()
                .query_map([], |row| Ok(SummandDoc { data: row.get(1)? }))
                .unwrap()
                .map(|doc| {
                    let summand: Summand = serde_json::from_str(&doc.unwrap().data).unwrap();
                    summand
                })
                .collect();
            return summands;
        } else {
            return vec![];
        }
    }
}
