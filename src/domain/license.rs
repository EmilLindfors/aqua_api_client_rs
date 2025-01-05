use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    api::{Endpoint, IntoQueryParams, NoQuery},
    generated::akvakulturregister::{LicenseDetail, SiteConnectionForLicense},
};

pub struct LicenseSiteConnectionEndpoint;

impl Endpoint for LicenseSiteConnectionEndpoint {
    type Query = NoQuery;
    type Response = SitesConnectionForLicenseResponse;
    type PathParams = i32;

    fn path(params: &Self::PathParams) -> String {
        format!("/api/v1/licenses/{}/site-connections", params)
    }
}

pub struct SitesConnectionForLicenseResponse(pub Vec<SiteConnectionForLicense>);

impl TryFrom<ureq::Response> for SitesConnectionForLicenseResponse {
    type Error = crate::error::Error;

    fn try_from(response: ureq::Response) -> Result<Self, Self::Error> {
        let sites: Vec<SiteConnectionForLicense> = response.into_json()?;
        Ok(SitesConnectionForLicenseResponse(sites))
    }
}

// License endpoint definition
pub struct LicenseEndpoint;

impl Endpoint for LicenseEndpoint {
    type Query = NoQuery;
    type Response = LicenseDetailResponse;
    type PathParams = i32;

    fn path(params: &Self::PathParams) -> String {
        format!("/api/v1/licenses/{}", params)
    }
}

pub struct LicenseDetailResponse(pub LicenseDetail);

impl TryFrom<ureq::Response> for LicenseDetailResponse {
    type Error = crate::error::Error;

    fn try_from(response: ureq::Response) -> Result<Self, Self::Error> {
        let license: LicenseDetail = response.into_json()?;
        Ok(LicenseDetailResponse(license))
    }
}

pub struct LicensesEndpoint;

impl Endpoint for LicensesEndpoint {
    type Query = LicenseQuery;
    type Response = LicensesDetailResponse;
    type PathParams = ();

    fn path(_: &Self::PathParams) -> String {
        "/api/v1/licenses".to_string()
    }
}

pub struct LicensesDetailResponse(pub Vec<LicenseDetail>);

impl TryFrom<ureq::Response> for LicensesDetailResponse {
    type Error = crate::error::Error;

    fn try_from(response: ureq::Response) -> Result<Self, Self::Error> {
        let licenses: Vec<LicenseDetail> = response.into_json()?;
        Ok(LicensesDetailResponse(licenses))
    }
}

#[derive(Debug, Default, Serialize)]
pub struct LicenseQuery {
    #[serde(rename = "license-nr")]
    license_nr: Option<String>,
    #[serde(rename = "original-license-nr")]
    original_license_nr: Option<String>,
    #[serde(rename = "legal-entity-nr-id")]
    legal_entity_nr_id: Option<String>,
    #[serde(rename = "legal-entity-nr")]
    legal_entity_nr: Option<String>,
    #[serde(rename = "legal-entity-name")]
    legal_entity_name: Option<String>,
    #[serde(rename = "portfolio-type")]
    portfolio_type: Option<PortfolioType>,
    tag: Option<String>,
    #[serde(rename = "not-tag")]
    not_tag: Option<String>,
    intention: Option<String>,
    produces: Option<String>,
    #[serde(rename = "species-code")]
    species_code: Option<String>,
    #[serde(rename = "species-type")]
    species_type: Option<String>,
    #[serde(rename = "species-comp-id")]
    species_comp_id: Option<String>,
    #[serde(rename = "municipality-code")]
    municipality_code: Option<String>,
    #[serde(rename = "county-code")]
    county_code: Option<String>,
    #[serde(rename = "production-area-code")]
    production_area_code: Option<String>,
    #[serde(rename = "valid-from")]
    valid_from: Option<DateTime<Utc>>,
    #[serde(rename = "registered-from")]
    registered_from: Option<DateTime<Utc>>,
    #[serde(rename = "temporary-until")]
    temporary_until: Option<DateTime<Utc>>,
    #[serde(rename = "temporary-from")]
    temporary_from: Option<DateTime<Utc>>,
    range: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PortfolioType {
    Master,
    Portfolio,
    Children,
}

impl LicenseQuery {
    pub fn builder() -> LicenseQueryBuilder {
        LicenseQueryBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct LicenseQueryBuilder {
    query: LicenseQuery,
}

impl LicenseQueryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn license_nr<S: Into<String>>(mut self, nr: S) -> Self {
        self.query.license_nr = Some(nr.into());
        self
    }

    pub fn original_license_nr<S: Into<String>>(mut self, nr: S) -> Self {
        self.query.original_license_nr = Some(nr.into());
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

    pub fn legal_entity_name<S: Into<String>>(mut self, name: S) -> Self {
        self.query.legal_entity_name = Some(name.into());
        self
    }

    pub fn portfolio_type(mut self, portfolio_type: PortfolioType) -> Self {
        self.query.portfolio_type = Some(portfolio_type);
        self
    }

    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.query.tag = Some(tag.into());
        self
    }

    pub fn not_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.query.not_tag = Some(tag.into());
        self
    }

    pub fn intention<S: Into<String>>(mut self, intention: S) -> Self {
        self.query.intention = Some(intention.into());
        self
    }

    pub fn produces<S: Into<String>>(mut self, produces: S) -> Self {
        self.query.produces = Some(produces.into());
        self
    }

    pub fn species_code<S: Into<String>>(mut self, code: S) -> Self {
        self.query.species_code = Some(code.into());
        self
    }

    pub fn species_type<S: Into<String>>(mut self, species_type: S) -> Self {
        self.query.species_type = Some(species_type.into());
        self
    }

    pub fn species_comp_id<S: Into<String>>(mut self, id: S) -> Self {
        self.query.species_comp_id = Some(id.into());
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

    pub fn temporary_until(mut self, date: DateTime<Utc>) -> Self {
        self.query.temporary_until = Some(date);
        self
    }

    pub fn temporary_from(mut self, date: DateTime<Utc>) -> Self {
        self.query.temporary_from = Some(date);
        self
    }

    pub fn range<S: Into<String>>(mut self, range: S) -> Self {
        self.query.range = Some(range.into());
        self
    }

    pub fn build(self) -> LicenseQuery {
        self.query
    }
}

impl IntoQueryParams for LicenseQuery {
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
