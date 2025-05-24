use std::fs::File;
use std::path::Path;

use sqlx::{SqlitePool};
use sqlx::Row;
use summand_app::summand::{Summand};
use uuid::Uuid;

use crate::data_adapter::DataAdapter;

#[derive(Debug)]
pub struct SQLxDataAdapter {
    client: SqlitePool,
}

impl DataAdapter<SQLxDataAdapter> for SQLxDataAdapter {
    async fn new() -> SQLxDataAdapter {
        let app_dir = "./";
        
        if !Path::new(&(app_dir.to_string() + "summand.db")).exists() {
            let database_file = File::create(format!("{}summand.db", app_dir));
        }

        let pool = SqlitePool::connect("sqlite:summand.db").await;
        match pool {
            Ok(client) => {
                println!("Database connection established successfully.");

                sqlx::query( "CREATE TABLE IF NOT EXISTS summand (
                        id TEXT PRIMARY KEY,
                        data TEXT
                    )")
                    .execute(&client)
                    .await
                    .unwrap();
                
                println!("Table created successfully.");

                return SQLxDataAdapter { client };
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
        let summand_json = serde_json::to_string(summand).unwrap();
        let result = sqlx::query("UPDATE summand SET data = $1 WHERE id = $2")
            .bind(summand_json)
            .bind(id.to_string())
            .execute(&self.client)
            .await
            .unwrap();
        println!("Updated {} rows", result.rows_affected());
    }

    async fn remove(&self, id: Uuid) {
        let result = sqlx::query("DELETE FROM summand WHERE id = $1")
            .bind(id.to_string())
            .execute(&self.client)
            .await
            .unwrap();
        println!("Deleted {} rows", result.rows_affected());
    }

    async fn find(&self, name: String) -> Option<Summand> {
        let rows_result = sqlx::query("SELECT data FROM summand WHERE json_extract(data, '$.name') = $1")
            .bind(name)
            .fetch_all(&self.client)
            .await;
        
        if let Ok(ref rows) = rows_result {
            println!("rows_result: {} rows fetched", rows.len());
        }
        match rows_result {
            Ok(rows) => {
                for row in rows {
                    if let Ok(data) = row.try_get::<String, _>("data") {
                        if let Ok(summand) = serde_json::from_str::<Summand>(&data) {
                            return Some(summand);
                        }
                    }
                }
                None
            }
            Err(e) => {
                eprintln!("Database error: {:?}", e);
                None
            }
        }
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
