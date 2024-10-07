use std::{env, error::Error};

use sqlx::{mysql::MySqlPoolOptions, pool::PoolConnection, prelude::Type, Connection, Encode, MySql, MySqlConnection, Pool };


#[derive(Type)]
pub enum Status {
    Paid,
    Waiting
}

pub async fn create_connection() -> Result<Pool<MySql>, Box<dyn Error>> {
    let url = env::var("DB_URL").expect("Set DB_URL as a envvar.");
    let user = env::var("DB_USER").expect("Set DB_USER as envvar.");
    let passwd = env::var("DB_PASS").expect("Set DB_PASS as envvar.");

    let db = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&format!("mysql://{user}:{passwd}@{url}/video_transcribe"))
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

pub async fn create_plan(conn: &Pool<MySql>, user_id: u32, plan_id: u32, status: Status )-> Result<(), Box<dyn Error>> {

    let trans = conn.begin().await?;
    let _ = sqlx::query("INSERT INTO log_pagamentos (user_id, plan_id, status, created_at) VALUES (?, ?, ?, UNIX_TIMESTAMP(NOW()));")
        .bind(user_id)
        .bind(plan_id)
        .bind(status)
        .execute(conn)
        .await?;

    trans.commit().await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenv;
    

    #[tokio::test]
    async fn testa_plano() {
        dotenv::dotenv().ok();

        let conn = create_connection().await.unwrap();

        create_plan(&conn, 1, 2, Status::Waiting).await.unwrap();
    }
}