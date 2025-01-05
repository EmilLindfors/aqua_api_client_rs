use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};

use crate::domain::{
    api::{Endpoint, IntoQueryParams, NoQuery},
    generated::fish_health::{
        BwApiApiFishhealthLocalityModelsLocalityNameIdMunicipalityDto,
        BwApiApiFishhealthLocalityModelsLocalitySeaTemperatureGraphDataDto,
    },
};

use super::lice::LocalityYearParams;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocalitySearchQuery {
    pub id: Option<i32>,
    pub name: Option<String>,
}

impl From<Option<LocalitySearchQuery>> for LocalitySearchQuery {
    fn from(query: Option<LocalitySearchQuery>) -> Self {
        query.unwrap_or_default()
    }
}

impl From<i32> for LocalitySearchQuery {
    fn from(id: i32) -> Self {
        LocalitySearchQuery {
            id: Some(id),
            name: None,
        }
    }
}

impl From<&str> for LocalitySearchQuery {
    fn from(name: &str) -> Self {
        LocalitySearchQuery {
            id: None,
            name: Some(name.to_string()),
        }
    }
}

impl IntoQueryParams for LocalitySearchQuery {
    fn into_query_params(self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(id) = self.id {
            params.insert("id".to_string(), id.to_string());
        }
        if let Some(name) = self.name {
            params.insert("name".to_string(), name);
        }
        params
    }
}

pub struct LocalitesEndpoint;

impl Endpoint for LocalitesEndpoint {
    type Query = LocalitySearchQuery;
    type Response = LocalitiesResponse;
    type PathParams = bool;

    fn path(params: &Self::PathParams) -> String {
        if *params {
            "/fishhealth/localitieswithsalmonoids".to_string()
        } else {
            "/fishhealth/localities".to_string()
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct LocalitiesResponse(
    pub Vec<BwApiApiFishhealthLocalityModelsLocalityNameIdMunicipalityDto>,
);

impl TryFrom<ureq::Response> for LocalitiesResponse {
    type Error = crate::error::Error;

    fn try_from(response: ureq::Response) -> Result<Self, Self::Error> {
        let localities: Vec<BwApiApiFishhealthLocalityModelsLocalityNameIdMunicipalityDto> =
            response.into_json()?;
        Ok(LocalitiesResponse(localities))
    }
}

pub struct LocalitySeaTemperatureEndpoint;

impl Endpoint for LocalitySeaTemperatureEndpoint {
    type Query = NoQuery;
    type Response = LocalitySeaTemperatureResponse;
    type PathParams = LocalityYearParams;

    fn path(params: &Self::PathParams) -> String {
        format!(
            "/fishhealth/locality/{}/seatemperature/{}",
            params.locality_no, params.year
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct LocalitySeaTemperatureResponse(
    pub BwApiApiFishhealthLocalityModelsLocalitySeaTemperatureGraphDataDto,
);

impl TryFrom<ureq::Response> for LocalitySeaTemperatureResponse {
    type Error = crate::error::Error;

    fn try_from(response: ureq::Response) -> Result<Self, Self::Error> {
        let locality: BwApiApiFishhealthLocalityModelsLocalitySeaTemperatureGraphDataDto =
            response.into_json()?;
        Ok(LocalitySeaTemperatureResponse(locality))
    }
}

impl fmt::Display for LocalitySeaTemperatureResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = &self.0;

        // Header with metadata
        writeln!(f, "Locality Sea Temperature Data")?;
        writeln!(f, "Locality No: {}", inner.locality_no.unwrap_or(-1))?;
        writeln!(f, "Year: {}", inner.year.unwrap_or(-1))?;
        writeln!(
            f,
            "Temperature Range: {:.1}°C - {:.1}°C",
            inner.min_sea_temperature.unwrap_or(f32::NAN),
            inner.max_sea_temperature.unwrap_or(f32::NAN)
        )?;
        writeln!(f)?;

        // Table header
        writeln!(f, "│ Week │ Temperature │ Reported │")?;
        writeln!(f, "├──────┼────────────┼──────────┤")?;

        // Sort data by week for consistent display
        let mut data = inner.data.clone();
        data.sort_by_key(|d| d.week.unwrap_or(-1));

        // Table rows
        for record in data.iter() {
            writeln!(
                f,
                "│ {:4} │ {:>9.1}°C │ {:>8} │",
                record.week.unwrap_or(-1),
                record.sea_temperature.unwrap_or(f32::NAN),
                record.has_reported.unwrap_or(false)
            )?;
        }

        // Table footer if data exists
        if !data.is_empty() {
            writeln!(f, "└──────┴────────────┴──────────┘")?;
        }

        Ok(())
    }
}
