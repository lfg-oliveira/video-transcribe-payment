use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::services::pix_service::{ CobImediataReq, CobrancaResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobPixResponse {
    pub calendario: Calendario,
    pub txid: String,
    pub revisao: i64,
    pub loc: Loc,
    pub location: String,
    pub status: String,
    pub devedor: Devedor,
    pub valor: Valor,
    pub chave: String,
    pub solicitacao_pagador: String,
    pub info_adicionais: Vec<InfoAdicionai>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Calendario {
    pub criacao: String,
    pub expiracao: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Loc {
    pub id: i64,
    pub location: String,
    pub tipo_cob: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Devedor {
    pub cnpj: String,
    pub nome: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Valor {
    pub original: String,
    pub modalidade_alteracao: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoAdicionai {
    pub nome: String,
    pub valor: String,
}


pub async fn pix_api_call(cob: CobImediataReq) -> Result<CobrancaResponse, reqwest::Error>  {
    let client = Client::new();

    let response = client.post("baas-api-sandbox.c6bank.info/v2/pix/cob")
        .json(&cob)
        .send()
        .await?
        .json()
        .await;

    response
}
