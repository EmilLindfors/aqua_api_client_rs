use crate::error::Error;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

fn request_inner<T: TryFrom<ureq::Response, Error = crate::error::Error>>(
    url: &str,
    params: Option<HashMap<String, String>>,
    token: Option<String>,
) -> Result<T, Error> {
    let mut url = url.to_string();

    if let Some(params) = params {
        if !params.is_empty() {
            url.push('?');
            url.push_str(
                &params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&"),
            );
        }
    }

    let res = if let Some(token) = token {
        ureq::get(&url)
            .set("Authorization", &format!("Bearer {}", token))
            .call()?
    } else {
        ureq::get(&url).call()?
    };

    if !res.status() == 200 {
        let status = res.status();
        let text = res.into_string()?;
        return Err(Error::Token(format!(
            "Token request failed: {} - {}",
            status, text
        )));
    }

    res.try_into()
}

pub trait EndpointRequest {
    fn base_url(&self) -> &str;
    fn authenticated_request<E: Endpoint>(
        &self,
        params: &E::PathParams,
        query: Option<E::Query>,
        token: Option<String>,
    ) -> Result<E::Response, Error> {
        request_inner(
            &format!("{}{}", self.base_url(), E::path(params)),
            query.map(|q| q.into_query_params()),
            token,
        )
    }
    fn request<E: Endpoint>(&self, params: &E::PathParams) -> Result<E::Response, Error> {
        request_inner(
            &format!("{}{}", self.base_url(), E::path(params)),
            None,
            None,
        )
    }
    fn query_request<E: Endpoint>(
        &self,
        params: &E::PathParams,
        query: Option<E::Query>,
    ) -> Result<E::Response, Error> {
        request_inner(
            &format!("{}{}", self.base_url(), E::path(params)),
            query.map(|q| q.into_query_params()),
            None,
        )
    }
}

pub trait Api {
    fn base_url(&self) -> &str;
    fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        let res = ureq::get(url).call()?;

        if !res.status() == 200 {
            let status = res.status();
            let text = res.into_string()?;
            return Err(Error::Token(format!(
                "Token request failed: {} - {}",
                status, text
            )));
        }

        Ok(res.into_json()?)
    }

    fn get_with_token<T: DeserializeOwned>(&self, url: &str, token: &str) -> Result<T, Error> {
        let res = ureq::get(url)
            .set("Authorization", &format!("Bearer {}", token))
            .call()?;

        if !res.status() == 200 {
            let status = res.status();
            let text = res.into_string()?;
            return Err(Error::Token(format!(
                "Token request failed: {} - {}",
                status, text
            )));
        }

        Ok(res.into_json()?)
    }
}

// Trait for query parameters
pub trait IntoQueryParams {
    fn into_query_params(self) -> HashMap<String, String>;
}

// Empty query type for endpoints without query parameters
#[derive(Debug, Default)]
pub struct NoQuery;

impl IntoQueryParams for NoQuery {
    fn into_query_params(self) -> HashMap<String, String> {
        HashMap::new()
    }
}

// Trait for endpoints
pub trait Endpoint {
    type Query: IntoQueryParams;
    type Response: TryFrom<ureq::Response, Error = crate::error::Error>;
    type PathParams;

    fn path(params: &Self::PathParams) -> String;
}
