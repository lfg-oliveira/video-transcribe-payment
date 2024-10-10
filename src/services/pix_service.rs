use std::{env, error::Error};

use axum::{http::{HeaderMap, HeaderValue}, response::{IntoResponse, Response}, routing::post, Json};
use mercadopago_sdk_rust::{ common_types::PersonalIdentification, payments::requests::{AdditionalInfo, BuyerEntityType, BuyerIdentification, BuyerType, CreatePaymentPayload, DocumentType, Order}, preferences::responses::CheckoutProPreferencesResponse, MercadoPagoSDKBuilder};
use reqwest::{Client, Method, Request, RequestBuilder, Url};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct BBAccessToken {
    access_token: String,
    expires_in: i64,
    scope: String,
    token_type: String,
}

impl BBAccessToken {
    pub async fn new() -> serde_json::Value {
        let basic = format!("Basic {}", env::var("BASIC_TOKEN").expect("BASIC_TOKEN envvar not found!"));

        let req = "https://oauth.hm.bb.com.br/oauth/token";

        let client = Client::builder();
        let mut headers = HeaderMap::new();

        headers.insert("Authorization", HeaderValue::from_str(&basic).unwrap());
        headers.insert("Content-Type", HeaderValue::from_str("application/x-www-form-urlencoded").unwrap());
        let client = client.default_headers(headers).build().unwrap();

        
        let response = client.post(req)
        .query(&[("grant_type", "client_credentials")])
        .send()
        .await
        .unwrap();

        let resp = response.json().await.unwrap();

        print!("{:#?}", resp);

        resp
    }

    pub async fn refresh_token(&mut self) {
        let new = Self::new().await;

        // self.access_token = new.get(index);
    }

    pub fn get_token(&self) -> String {
        format!("{} {}", self.token_type, self.access_token)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BBGerarPix {
    pub calendario: Calendario,
    pub devedor: Option<Devedor>,
    pub valor: Valor,
    pub chave: String,
    pub solitacao_pagador: String,
    pub info_adicionais: Option<Vec<InfoAdicionai>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Calendario {
    pub expiracao: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Devedor {
    pub cpf: String,
    pub nome: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Valor {
    pub original: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoAdicionai {
    pub nome: String,
    pub valor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BBGeneratePixResponse {
    pub calendario: Calendario,
    pub devedor: Devedor,
    pub valor: Valor,
    pub chave: String,
    pub solicitacao_pagador: String,
    pub info_adicionais: Vec<InfoAdicionai>,
    pub txid: String,
    pub revisao: i64,
    pub location: String,
    pub status: String,
    #[serde(rename = "pixCopiaECola")]
    pub pix_copia_ecola: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VTRequest {
    pub devedor: Option<Devedor>,
    pub valor: Valor
}

impl From<VTRequest> for BBGerarPix {
    fn from(value: VTRequest) -> Self {
        Self {
            calendario: Calendario {
                expiracao: 3600
            },
            chave: "1234567890".to_string(),
            devedor: value.devedor,
            valor: value.valor,
            ..Default::default()
        }
    }
}

pub async fn gera_qr_code(data: VTRequest) -> Result<String, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    let acc_tk = BBAccessToken::new().await;
    let dev_key = env::var("DEV_KEY").expect("DEV_KEY envvar not found");
    // headers.append("Authorization", HeaderValue::from_str(&acc_tk.get_token()).unwrap());
    let client = Client::builder()
    .default_headers(headers)
    .build().unwrap();

    let response: BBGeneratePixResponse = client.post(format!("https://api.hm.bb.com.br/pix/v2/cob?gw-dev-app-key={dev_key}"))
    .json(&BBGerarPix::from(data))
    .send()
    .await?
    .json()
    .await?;

    Ok(response.pix_copia_ecola)
}

#[cfg(test)]
mod tests {

    use dotenv::dotenv;

    use super::*;

    #[tokio::test]
    async fn pix_api_test() {
        dotenv().ok();

        let json = VTRequest {
            devedor: None,
            valor: Valor {
                original: "15.30".to_string()
            }
        };
        print!("{}", gera_qr_code(json).await.unwrap());
    }
}
