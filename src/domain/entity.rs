use std::collections::HashMap;

use serde::Serialize;

use super::{
    api::{Endpoint, IntoQueryParams, NoQuery},
    generated::akvakulturregister::Entity,
};

pub struct EntityEndpoint;

impl Endpoint for EntityEndpoint {
    type Query = NoQuery;
    type Response = EntityResponse;
    type PathParams = i32;

    fn path(params: &Self::PathParams) -> String {
        format!("/api/v1/entities/{}", params)
    }
}

pub struct EntityResponse(pub Entity);

impl TryFrom<ureq::Response> for EntityResponse {
    type Error = crate::error::Error;

    fn try_from(response: ureq::Response) -> Result<Self, Self::Error> {
        let entity: Entity = response.into_json()?;
        Ok(EntityResponse(entity))
    }
}

pub struct EntitiesEndpoint;

impl Endpoint for EntitiesEndpoint {
    type Query = EntityQuery;
    type Response = EntitiesResponse;
    type PathParams = ();

    fn path(_: &Self::PathParams) -> String {
        "/api/v1/entities".to_string()
    }
}

pub struct EntitiesResponse(pub Vec<Entity>);

impl TryFrom<ureq::Response> for EntitiesResponse {
    type Error = crate::error::Error;

    fn try_from(response: ureq::Response) -> Result<Self, Self::Error> {
        let entities: Vec<Entity> = response.into_json()?;
        Ok(EntitiesResponse(entities))
    }
}

#[derive(Debug, Default, Serialize)]
pub struct EntityQuery {
    #[serde(rename = "entity-nr-id")]
    entity_nr_id: Option<String>,
    #[serde(rename = "entity-nr")]
    entity_nr: Option<String>,
    name: Option<String>,
    #[serde(rename = "zip-code")]
    zip_code: Option<String>,
    #[serde(rename = "zip-name")]
    zip_name: Option<String>,
    #[serde(rename = "county-code")]
    county_code: Option<String>,
    #[serde(rename = "county-name")]
    county_name: Option<String>,
    #[serde(rename = "country-code")]
    country_code: Option<String>,
    #[serde(rename = "country-name")]
    country_name: Option<String>,
    range: Option<String>,
}

impl EntityQuery {
    pub fn builder() -> EntityQueryBuilder {
        EntityQueryBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct EntityQueryBuilder {
    query: EntityQuery,
}

impl EntityQueryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entity_nr_id<S: Into<String>>(mut self, id: S) -> Self {
        self.query.entity_nr_id = Some(id.into());
        self
    }

    pub fn entity_nr<S: Into<String>>(mut self, nr: S) -> Self {
        self.query.entity_nr = Some(nr.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.query.name = Some(name.into());
        self
    }

    pub fn zip_code<S: Into<String>>(mut self, code: S) -> Self {
        self.query.zip_code = Some(code.into());
        self
    }

    pub fn zip_name<S: Into<String>>(mut self, name: S) -> Self {
        self.query.zip_name = Some(name.into());
        self
    }

    pub fn county_code<S: Into<String>>(mut self, code: S) -> Self {
        self.query.county_code = Some(code.into());
        self
    }

    pub fn county_name<S: Into<String>>(mut self, name: S) -> Self {
        self.query.county_name = Some(name.into());
        self
    }

    pub fn country_code<S: Into<String>>(mut self, code: S) -> Self {
        self.query.country_code = Some(code.into());
        self
    }

    pub fn country_name<S: Into<String>>(mut self, name: S) -> Self {
        self.query.country_name = Some(name.into());
        self
    }

    pub fn range<S: Into<String>>(mut self, range: S) -> Self {
        self.query.range = Some(range.into());
        self
    }

    pub fn build(self) -> EntityQuery {
        self.query
    }
}

impl IntoQueryParams for EntityQuery {
    fn into_query_params(self) -> HashMap<String, String> {
        serde_qs::to_string(&self)
            .unwrap_or_default()
            .split('&')
            .filter_map(|pair| {
                let mut split = pair.split('=');
                match (split.next(), split.next()) {
                    (Some(key), Some(value)) => Some((
                        key.to_string(),
                        urlencoding::decode(value)
                            .unwrap_or_else(|_| std::borrow::Cow::Borrowed(value))
                            .into_owned(),
                    )),
                    _ => None,
                }
            })
            .collect()
    }
}
