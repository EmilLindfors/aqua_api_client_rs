use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;

use super::generated::akvakulturregister::SiteForLegalEntity;
use super::{
    api::{Endpoint, IntoQueryParams, NoQuery},
    generated::akvakulturregister::Site,
};

pub struct SiteEndpoint;

impl Endpoint for SiteEndpoint {
    type Query = NoQuery;
    type Response = SiteResponse;
    type PathParams = i32;

    fn path(params: &Self::PathParams) -> String {
        format!("/api/v1/sites/{}", params)
    }
}

pub struct SiteResponse(pub Site);

impl TryFrom<ureq::Response> for SiteResponse {
    type Error = crate::error::Error;

    fn try_from(response: ureq::Response) -> Result<Self, Self::Error> {
        let site: Site = response.into_json()?;
        Ok(SiteResponse(site))
    }
}

pub enum EntityNumber {
    /// Legal entity number, 9 digits
    Organization(i32),
    /// Person number, 11 digits
    Person(i32),
}

impl TryFrom<i32> for EntityNumber {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 100_000_000 {
            Ok(EntityNumber::Organization(value))
        } else if value < 1_000_000_000 {
            Ok(EntityNumber::Person(value))
        } else {
            Err("Invalid entity number")
        }
    }
}

impl fmt::Display for EntityNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntityNumber::Organization(nr) => write!(f, "{:09}", nr),
            EntityNumber::Person(nr) => write!(f, "{:011}", nr),
        }
    }
}

pub struct SitesForEntityEndpoint;

impl Endpoint for SitesForEntityEndpoint {
    type Query = SitesForEntityQuery;
    type Response = SitesForLegalEntityResponse;
    type PathParams = EntityNumber;

    fn path(params: &Self::PathParams) -> String {
        format!("/api/v1/entities/sites-by-entity-nr/{params}")
    }
}

pub struct SitesForLegalEntityResponse(pub Vec<SiteForLegalEntity>);

impl TryFrom<ureq::Response> for SitesForLegalEntityResponse {
    type Error = crate::error::Error;

    fn try_from(response: ureq::Response) -> Result<Self, Self::Error> {
        let sites: Vec<SiteForLegalEntity> = response.into_json()?;
        Ok(SitesForLegalEntityResponse(sites))
    }
}

pub struct SitesEndpoint;

impl Endpoint for SitesEndpoint {
    type Query = SiteQuery;
    type Response = SitesResponse;
    type PathParams = ();

    fn path(_params: &Self::PathParams) -> String {
        format!("/api/v1/sites")
    }
}

pub struct SitesResponse(pub Vec<Site>);

impl TryFrom<ureq::Response> for SitesResponse {
    type Error = crate::error::Error;

    fn try_from(response: ureq::Response) -> Result<Self, Self::Error> {
        let sites: Vec<Site> = response.into_json()?;
        Ok(SitesResponse(sites))
    }
}

#[derive(Debug, Default, Serialize)]
pub struct SitesForEntityQuery {
    /// Henter ut angitt utvalg av lokaliteter. Må være i følgende format: 0-4
    range: Option<String>,
    /// Tar med tilknytninger til lokaliteter som benyttes av den angitte juridisk enheten
    include_all_connections: Option<String>,
    #[serde(rename = "activity")]
    activity_type: Option<ActivityType>,
    #[serde(rename = "operation")]
    operation_type: Option<OperationType>,
}

impl IntoQueryParams for SitesForEntityQuery {
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

#[derive(Debug, Default, Serialize)]
pub struct SiteQuery {
    nr: Option<String>,
    legal_entity_nr_id: Option<String>,
    legal_entity_nr: Option<String>,
    license_nr: Option<String>,
    name: Option<String>,
    placement: Option<String>,
    water: Option<String>,
    species_type: Option<String>,
    municipality_code: Option<String>,
    county_code: Option<String>,
    production_area_code: Option<String>,
    valid_from: Option<DateTime<Utc>>,
    registered_from: Option<DateTime<Utc>>,
    range: Option<String>,
    #[serde(rename = "activity")]
    activity_type: Option<ActivityType>,
    #[serde(rename = "operation")]
    operation_type: Option<OperationType>,
}

impl IntoQueryParams for SiteQuery {
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityType {
    Any,
    Commercial,
    NonCommercial,
    Slaughtery,
    NonSlaughtery,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OperationType {
    Any,
    Conventional,
    Colocation,
    JointOps,
}

impl SiteQuery {
    pub fn builder() -> SiteQueryBuilder {
        SiteQueryBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SiteQueryBuilder {
    query: SiteQuery,
}

impl SiteQueryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn nr<S: Into<String>>(mut self, nr: S) -> Self {
        self.query.nr = Some(nr.into());
        self
    }

    pub fn legal_entity_nr_id<S: Into<String>>(mut self, id: S) -> Self {
        self.query.legal_entity_nr_id = Some(id.into());
        self
    }

    pub fn legal_entity_nr<S: Into<String>>(mut self, nr: S) -> Self {
        self.query.legal_entity_nr = Some(nr.into());
        self
    }

    pub fn license_nr<S: Into<String>>(mut self, nr: S) -> Self {
        self.query.license_nr = Some(nr.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.query.name = Some(name.into());
        self
    }

    pub fn placement<S: Into<String>>(mut self, placement: S) -> Self {
        self.query.placement = Some(placement.into());
        self
    }

    pub fn water<S: Into<String>>(mut self, water: S) -> Self {
        self.query.water = Some(water.into());
        self
    }

    pub fn species_type<S: Into<String>>(mut self, species: S) -> Self {
        self.query.species_type = Some(species.into());
        self
    }

    pub fn municipality_code<S: Into<String>>(mut self, code: S) -> Self {
        self.query.municipality_code = Some(code.into());
        self
    }

    pub fn county_code<S: Into<String>>(mut self, code: S) -> Self {
        self.query.county_code = Some(code.into());
        self
    }

    pub fn production_area_code<S: Into<String>>(mut self, code: S) -> Self {
        self.query.production_area_code = Some(code.into());
        self
    }

    pub fn valid_from(mut self, date: DateTime<Utc>) -> Self {
        self.query.valid_from = Some(date);
        self
    }

    pub fn registered_from(mut self, date: DateTime<Utc>) -> Self {
        self.query.registered_from = Some(date);
        self
    }

    pub fn range<S: Into<String>>(mut self, range: S) -> Self {
        self.query.range = Some(range.into());
        self
    }

    pub fn activity_type(mut self, activity: ActivityType) -> Self {
        self.query.activity_type = Some(activity);
        self
    }

    pub fn operation_type(mut self, operation: OperationType) -> Self {
        self.query.operation_type = Some(operation);
        self
    }

    pub fn build(self) -> SiteQuery {
        self.query
    }
}

impl SitesForEntityQuery {
    pub fn builder() -> SitesForEntityQueryBuilder {
        SitesForEntityQueryBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SitesForEntityQueryBuilder {
    query: SitesForEntityQuery,
}

impl SitesForEntityQueryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn range(mut self, start: u32, end: u32) -> Self {
        self.query.range = Some(format!("{}-{}", start, end));
        self
    }

    pub fn include_all_connections(mut self, bool: bool) -> Self {
        if bool {
            self.query.include_all_connections = Some("true".to_string());
        } else {
            self.query.include_all_connections = Some("false".to_string());
        }
        self
    }

    pub fn activity_type(mut self, activity: ActivityType) -> Self {
        self.query.activity_type = Some(activity);
        self
    }

    pub fn operation_type(mut self, operation: OperationType) -> Self {
        self.query.operation_type = Some(operation);
        self
    }

    pub fn build(self) -> SitesForEntityQuery {
        self.query
    }
}
