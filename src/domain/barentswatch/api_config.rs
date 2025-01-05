use lazy_static::lazy_static;
use serde::Deserialize;
use std::sync::Mutex;

use crate::{domain::api::{Endpoint, NoQuery}, error::Error};

lazy_static! {
    pub static ref TOKEN_CACHE: Mutex<Option<TokenResponse>> = Mutex::new(None);
}

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    token_type: String,
    scope: String,
}


pub struct JsonEndpoint;

impl Endpoint for JsonEndpoint {
    type Query = NoQuery;
    type Response = JsonResponse;
    type PathParams = String;

    fn path(path: &Self::PathParams) -> String {
      path.clone()
    }
}


pub struct JsonResponse(pub serde_json::Value);

impl JsonResponse {
    pub fn into_inner(self) -> serde_json::Value {
        self.0
    }
}

impl TryFrom<ureq::Response> for JsonResponse {
    type Error = crate::error::Error;

    fn try_from(res: ureq::Response) -> Result<Self, Self::Error> {
        let status = res.status();
       if status == 200 {
           let json = res.into_json()?;
           Ok(JsonResponse(json))
       } else {
           let text = res.into_string()?;
           Err(Error::Api(format!("Request failed: {} - {}", status, text)))
       }
    }
}