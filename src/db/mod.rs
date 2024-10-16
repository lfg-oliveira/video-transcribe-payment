use std::{env, error::Error};

use sqlx::{mysql::MySqlConnectOptions, prelude::Type, ConnectOptions, Connection, MySqlConnection };


#[derive(Type)]
pub enum Status {
    Paid,
    Waiting
}

pub async fn create_connection() -> Result<MySqlConnection, Box<dyn Error>> {
    let url = env::var("DB_URL").expect("Set DB_URL as a envvar.");
    let user = env::var("DB_USER").expect("Set DB_USER as envvar.");
    let _passwd = env::var("DB_PASS").expect("Set DB_PASS as envvar.");
    let port = env::var("DB_PORT").unwrap_or_else(|_| "3306".to_string());

    let db = MySqlConnection::connect(&format!("mysql://{user}@{url}:{port}/video_transcribe"))
        .await?;


    Ok(db)
}

// impl IntoValue for Status {
//     fn into_value(self) -> libsql::Result<libsql::Value> {
//         match self {
//             Self::Paid => Ok(libsql::Value::Text("Paid".to_string())),
//             Self::Waiting => Ok(libsql::Value::Text("Waiting".to_string()))
//         }
//     }
// }

pub async fn create_plan(conn: & mut MySqlConnection, user_id: u32, plan_id: u32, status: Status )-> Result<(), Box<dyn Error>> {

    let _ = sqlx::query("INSERT INTO log_pagamento (user_id, plan_id, status, created_at) VALUES (?, ?, ?, NOW());")
        .bind(user_id)
        .bind(plan_id)
        .bind(status)
        .execute(conn)
        .await?;



    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;


    #[tokio::test]
    async fn testa_plano() {
        dotenv::dotenv().ok();

        let mut conn = match create_connection().await {
            Ok(o) => o,
            Err(e) => panic!("Something went wrong establishing connection, {e}")
        };
        println!("Inserting now");
        create_plan(&mut conn, 1, 2, Status::Waiting).await.unwrap();
    }
}
