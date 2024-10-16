mod emv;
mod http_client;
mod services;
mod db;
use std::env;
use axum::{
    response::{IntoResponse, Response}, routing::{get, post}, Json, Router
};
use reqwest::StatusCode;
use tokio::net::TcpListener;
use services::pix_service::{gera_qr_code, VTRequest};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/gerarPix", post(string_qr_code));

    let listener = TcpListener::bind(env::var("PORT")
                                    .unwrap_or_else(|_| "3000".to_string()))
                                    .await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn string_qr_code(Json(payload): Json<VTRequest>) -> Response
{
    match gera_qr_code(payload).await {
        Ok(r) =>  r.into_response(),
        Err(_) => "Error with generating pix".into_response()
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::*;
    use db::{create_connection, create_plan, Status};
    use dotenv::dotenv;

    #[tokio::test]
    async fn main_test() -> Result<(), Box<dyn Error>>{
        dotenv().ok();
        let mut con = create_connection()
            .await?;

        create_plan(& mut con, 1, 1, Status::Waiting)
            .await?;
        Ok(())
    }
}
