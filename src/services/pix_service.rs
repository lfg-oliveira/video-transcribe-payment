use std::error::Error;

use axum::{response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};

use crate::{emv::get_qr_code_string,  http_client::pix_api_call};

#[derive(Deserialize, Serialize)]
struct Calendario {
    pub expiracao: u32
}

#[derive(Deserialize, Serialize)]
struct Devedor {
    pub nome: String,
    pub cpf: String
}

#[derive(Deserialize, Serialize)]
struct Valor {
    pub original: f64
}

#[derive(Deserialize, Serialize)]
pub struct CobImediata {
    pub calendario: Calendario,
    pub devedor: Devedor,
    pub valor: Valor,
    pub chave: String
}

#[derive(Deserialize, Serialize)]
pub struct CobImediataReq {
    pub devedor: Devedor,
    pub valor: Valor
}

impl CobImediata {
    /***
     *  Default expiration is 20m (1200s)
     */
    pub fn from_cob_imediata_req(cob: CobImediataReq) -> Self {
        Self {
            valor: cob.valor,
            calendario: Calendario { expiracao: 1200 },
            devedor: cob.devedor,
            chave: String::from("a1784014-7211-471d-812d-a8c656d8045b")
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobrancaResponse {
    pub calendario: CalendarioResponse,
    pub txid: String,
    pub loc: LocationResponse,
    pub status: String,
    pub devedor: Devedor,
    pub valor: Valor,
    pub chave: String,
    pub solicitacao_pagador: String,
}

#[derive(Deserialize, Debug)]
pub struct CalendarioResponse {
    pub criacao: String,
    pub expiracao: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct LocationResponse {
    pub id: i64,
    pub location: String,
    pub tipo_cob: String,
    pub criacao: String,
}

pub async fn gera_qr_code(Json(data): Json<CobImediataReq>) -> Result<Response, Box<dyn Error>> {
    let response = pix_api_call(data).await?;

    let pix_str = get_qr_code_string(response.valor.original, &response.loc.location, "VideoTranscribe");
    return Ok(pix_str.into_response())
}
