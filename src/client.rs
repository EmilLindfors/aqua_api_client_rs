use crate::error::{Error, Result};
use crate::domain::generated::fish_health::*;
use crate::domain::barentswatch::vessel::*;
use lazy_static::lazy_static;

use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};


#[derive(Clone)]
pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
}

impl Client {
    pub fn new() -> Self {
        Self {
            base_url: "https://www.barentswatch.no/bwapi/v1/geodata".to_string(),
            token_url: "https://id.barentswatch.no/connect/token".to_string(),
            config: None,
        }
    }

    pub fn with_config(config: ClientConfig) -> Self {
        Self {
            base_url: "https://www.barentswatch.no/bwapi/v1/geodata".to_string(),
            token_url: "https://id.barentswatch.no/connect/token".to_string(),
            config: Some(config),
        }
    }

    async fn get_token(&self) -> Result<TokenResponse> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| Error::Auth("Config required for authentication".to_string()))?;

        let res = ureq::post(&self.token_url)
            .send_form(&[("client_id", &config.client_id), ("scope", "api"), ("client_secret", &config.client_secret), ("grant_type", "client_credentials")])?;

        if !res.status() == 200 {
            let status = res.status();
            let text = res.into_string()?;
            return Err(Error::Token(format!(
                "Token request failed: {} - {}",
                status, text
            )));
        }

        Ok(res.into_json::<TokenResponse>()?)
    }

    async fn ensure_token(&self) -> Result<String> {
        let mut cache = TOKEN_CACHE
            .lock()
            .map_err(|_| Error::Token("Failed to acquire token cache lock".to_string()))?;

        if let Some((token, expiry)) = cache.as_ref() {
            if expiry.elapsed() < Duration::from_secs(3500) {
                return Ok(token.clone());
            }
        }

        let token_response = self.get_token().await?;
        let token = token_response.access_token;
        *cache = Some((token.clone(), Instant::now()));
        Ok(token)
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let token = self.ensure_token().await?;

        let res = ureq::get(&url).set("Authorization", &format!("Bearer {}", token)).call()?;

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

    // Fish Health endpoints
    pub async fn get_locality_report(
        &self,
        locality_no: i32,
        year: i32,
        week: i32,
    ) -> Result<BwApiApiFishhealthLocalityModelsLocalityReportV1> {
        self.get(&format!(
            "/fishhealth/locality/{}/{}/{}",
            locality_no, year, week
        ))
        .await
    }

    pub async fn get_locality_lice_count(
        &self,
        locality_no: i32,
        year: i32,
    ) -> Result<BwApiApiFishhealthLocalityModelsLocalityLiceGraphDataDto> {
        self.get(&format!(
            "/fishhealth/locality/{}/avgfemalelice/{}",
            locality_no, year
        ))
        .await
    }

    pub async fn get_locality_lice_distribution(
        &self,
        locality_no: i32,
        year: i32,
    ) -> Result<LocalityLiceDistribution> {
        let dist: BwApiApiFishhealthLocalityModelsLocalityLiceDistributionGraphDataDto = self
            .get(&format!(
                "/fishhealth/locality/{locality_no}/liceTypeDistribution/{year}"
            ))
            .await?;
        Ok(dist.into())
    }

    pub async fn get_locality_vessel_visits(
        &self,
        locality_no: i32,
        year: i32,
        week: Option<i32>,
    ) -> Result<LocalityVesselVisits> {
        if let Some(week) = week {
            let res: BwApiApiVesselTrackWeeksModelsVesselTrackLocalityWeek = self
                .get(&format!(
                    "/fishhealth/locality/{locality_no}/vessel/{year}/{week}"
                ))
                .await?;

            Ok(res.into())
        } else {
            let res: Vec<BwApiApiVesselTrackWeeksModelsVesselTrackLocalityWeek> = self
                .get(&format!("/fishhealth/locality/{locality_no}/vessel/{year}"))
                .await?;

            Ok(res.into())
        }
    }

    pub async fn get_locality_diseasezonehistory_history(
        &self,
        locality_no: i32,
        year: Option<i32>,
        week: Option<i32>,
    ) -> Result<BwApiApiFishhealthDiseaseHistoryModelsDiseaseZoneHistory> {
        let mut path = format!("/fishhealth/locality/diseasezonehistory/{}", locality_no);

        if let Some(year) = year {
            path.push_str(&format!("/{}", year));
        }

        if let Some(week) = week {
            path.push_str(&format!("/{}", week));
        }

        self.get(&path).await
    }

    pub async fn get_locality_treatments(
        &self,
        locality_no: i32,
        year: i32,
    ) -> Result<BwApiApiFishhealthLocalityModelsAllTreatmentsGraphDataDto> {
        self.get(&format!(
            "/fishhealth/locality/{}/liceTreatments/{}",
            locality_no, year
        ))
        .await
    }

    pub async fn get_localities(
        &self,
    ) -> Result<Vec<BwApiApiFishhealthLocalityModelsLocalityNameIdMunicipalityDto>> {
        self.get("/fishhealth/localities").await
    }

    pub async fn get_vessels(&self) -> Result<Vessels> {
        let res: Vec<BwApiApiVesselTrackWeeksModelsVesselInfo> =
            self.get(&format!("/fishhealth/vessels")).await?;

        Ok(res.into())
    }

    pub async fn get_localities_with_salmonoids(
        &self,
    ) -> Result<Vec<BwApiApiFishhealthLocalityModelsLocalityNameIdDto>> {
        self.get("/fishhealth/localitieswithsalmonoids").await
    }

    // Production area endpoints
    pub async fn get_production_area(
        &self,
        id: i32,
        year: i32,
        week: i32,
    ) -> Result<BwApiApiProductionAreasModelsProductionAreaWeek> {
        self.get(&format!("/productionarea/{}/{}/{}", id, year, week))
            .await
    }

    pub async fn get_production_areas(
        &self,
    ) -> Result<Vec<BwApiApiProductionAreasModelsProductionArea>> {
        self.get("/productionarea").await
    }

    // Disease zone endpoints
    pub async fn get_pd_zones(&self) -> Result<Vec<BwApiApiIlaPdsModelsPdZone>> {
        self.get("/fishhealth/pdzone").await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
