// use duckdb::{params, Connection};
// use summand_app::summand::{self, Summand};
// use uuid::Uuid;

// use crate::data_adapter::DataAdapter;

// #[derive(Debug)]
// pub struct DockdbDataAdapter {
//     client: Connection,
// }

// impl Clone for DockdbDataAdapter {
//     fn clone(&self) -> Self {
//         DockdbDataAdapter {
//             client: self.client.try_clone().unwrap(),
//         }
//     }
// }

// struct SummandDoc {
//     data: String,
// }

// impl DataAdapter<DockdbDataAdapter> for DockdbDataAdapter {
//     fn new() -> DockdbDataAdapter {
//         let connection = Connection::open("./summand.db3");
//         if let Ok(conn) = connection {
//             println!("Database connection established successfully.");
//             let result = conn.execute_batch(
//                 "CREATE TABLE IF NOT EXISTS summand1 (
//                     id TEXT,
//                     data TEXT
//                 )"
//             );

//             match result {
//                 Ok(_) => {
//                     println!("Table created successfully.");
//                 }
//                 Err(e) => {
//                     println!("Failed to create table: {}", e);
//                 }
//             }


//             return DockdbDataAdapter { client: conn };
//         } else {
//             return DockdbDataAdapter {
//                 client: Connection::open_in_memory().unwrap(),
//             };
//         }
//     }

//     fn create(&self, summand: &summand_app::summand::Summand) {
//         let summand_json = serde_json::to_string(summand).unwrap();
//         // let table = self
//         //     .client
//         //     .prepare("CREATE TABLE IF NOT EXISTS t1 (id VARCHAR, data VARCHAR)");
//         let result = self.client.execute("INSERT INTO summand1 (id, data) VALUES (?,?)", params![
//             "hello",
//             "hi"
//         ]);

//         match result {
//             Ok(_) => {
//                 println!("Data inserted successfully.");
//             }
//             Err(e) => {
//                 println!("Failed to insert data: {}", e);
//             }
//         }
//     }

//     fn update(&self, id: uuid::Uuid, summand: &summand_app::summand::Summand) {
//         let summand_json = serde_json::to_string(summand);
//         let _ = self.client.execute(
//             "INSERT INTO summand (id, data) VALUES (?, ?)",
//             params![summand.id.to_string(), summand_json.unwrap()],
//         );
//     }

//     fn remove(&self, id: uuid::Uuid) {
//         todo!()
//     }

//     fn find(&self, id: uuid::Uuid) -> Option<Summand> {
//         todo!()
//     }

//     fn find_all(&self) -> Vec<Summand> {
//         let summands_query = self.client.prepare("SELECT id,data FROM summand");
//         if summands_query.is_ok() {
//             let summands: Vec<Summand> = summands_query
//                 .unwrap()
//                 .query_map([], |row| Ok(SummandDoc { data: row.get(1)? }))
//                 .unwrap()
//                 .map(|doc| {
//                     let summand: Summand = serde_json::from_str(&doc.unwrap().data).unwrap();
//                     summand
//                 })
//                 .collect();
//             return summands;
//         } else {
//             return vec![];
//         }
//     }
// }
