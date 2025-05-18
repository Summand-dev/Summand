use sqlx::{Executor, SqlitePool};
use sqlx::Row;
use summand_app::summand::{self, Summand};
use uuid::Uuid;

use crate::data_adapter::DataAdapter;

#[derive(Debug)]
pub struct SQLxDataAdapter {
    client: SqlitePool,
}

// impl Clone for SQLxDataAdapter {
//     fn clone(&self) -> Self {
//         DockdbDataAdapter {
//             client: self.client.try_clone().unwrap(),
//         }
//     }
// }

struct SummandDoc {
    data: String,
}

impl DataAdapter<SQLxDataAdapter> for SQLxDataAdapter {
    async fn new() -> SQLxDataAdapter {
        let pool = SqlitePool::connect("sqlite:summand.db").await;
        match pool {
            Ok(client) => {
                println!("Database connection established successfully.");

                // client
                //     .execute(
                //         "CREATE TABLE IF NOT EXISTS summand (
                //         id TEXT PRIMARY KEY,
                //         data TEXT
                //     )",
                //     )
                //     .await
                //     .unwrap();
                // println!("Table created successfully.");

                let adapter = SQLxDataAdapter { client };
                let command = summand_app::command::Command::new(
                    "Echo",
                    Some("Echo a fixed value"),
                    "echo",
                    Some(vec![summand_app::argument::CommandArgument::new(
                        None,
                        Some("test"),
                    )]),
                );

                let summand =
                    summand_app::summand::Summand::new("test", Some("test"), Some(vec![command]));
                // adapter.create(&summand).await;
                adapter
            }
            Err(_e) => SQLxDataAdapter {
                client: SqlitePool::connect_lazy("sqlite::memory:").unwrap(),
            },
        }
    }

    async fn create(&self, summand: &Summand) {
        let summand_json = serde_json::to_string(summand).unwrap();
        print!("hi");
        let result = sqlx::query("INSERT INTO summand (id, data) VALUES ($1, $2)")
            .bind(summand.id.to_string())
            .bind(summand_json)
            .execute(&self.client)
            .await
            .unwrap();
        println!("Inserted {} rows", result.rows_affected());
    }

    async fn update(&self, id: Uuid, summand: &Summand) {
        todo!()
    }

    async fn remove(&self, id: Uuid) {
        todo!()
    }

    async fn find(&self, id: Uuid) -> Option<Summand> {
        todo!()
    }

    async fn find_all(&self) -> Vec<Summand> {
        let rows = sqlx::query("SELECT data FROM summand")
            .fetch_all(&self.client)
            .await;
        if let Ok(rows) = rows {
            let summands = rows
                .into_iter()
                .filter_map(|row| {
                    let data: String = row.try_get("data").ok()?;
                    serde_json::from_str::<Summand>(&data).ok()
                })
                .collect();
            return summands;
        } else {
            return vec![];
        }
    }
}
