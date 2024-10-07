use std::{env, error::Error};

use libsql::{params::{self, IntoValue}, Builder, Connection};

pub enum Status {
    Paid,
    Waiting
}

pub async fn create_connection() -> Result<Connection, Box<dyn Error>> {
    let url = env::var("TURSO_DB_URL").expect("Set TURSO_DB_URL as a envvar.");
    let token = env::var("TURSO_AUTH_TOKEN").expect("Set TURSO_AUTH_TOKEN as envvar.");

    let db = Builder::new_remote(url, token)
        .build()
        .await?;

    Ok(db.connect()?)
}

impl IntoValue for Status {
    fn into_value(self) -> libsql::Result<libsql::Value> {
        match self {
            Self::Paid => Ok(libsql::Value::Text("Paid".to_string())),
            Self::Waiting => Ok(libsql::Value::Text("Waiting".to_string()))
        }
    }
}

pub async fn create_plan(conn: &Connection, user_id: u32, plan_id: u32, status: Status, )-> Result<(), Box<dyn Error>> {

    conn.query("INSERT INTO log_pagamentos (user_id, plan_id, status, created_at) VALUES (?1, ?2, ?3, strftime('%s','now'))",
               libsql::params![user_id, plan_id, status]).await?;
    Ok(())
}
