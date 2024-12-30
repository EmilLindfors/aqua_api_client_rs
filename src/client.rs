use crate::error::{Error, Result};
use crate::fish_health::*;
use crate::vessel::*;
use lazy_static::lazy_static;
use reqwest::{Client as ReqwestClient, RequestBuilder};
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use std::vec;

#[derive(Serialize, Deserialize, Debug)]
pub struct LiceDistribution {
    locality_no: i32,
    year: i32,
    week: i32,
    avg_adult_female_lice: Option<f64>,
    avg_mobile_lice: Option<f64>,
    avg_stationary_lice: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct LocalityLiceDistribution(pub Vec<LiceDistribution>);

impl From<BwApiApiFishhealthLocalityModelsLocalityLiceDistributionGraphDataDto>
    for LocalityLiceDistribution
{
    fn from(dto: BwApiApiFishhealthLocalityModelsLocalityLiceDistributionGraphDataDto) -> Self {
        let data = dto
            .data
            .into_iter()
            .map(|x| LiceDistribution {
                locality_no: dto.locality_no.unwrap_or(0),
                year: dto.year.unwrap_or(0),
                week: x.week.unwrap_or(0),
                avg_adult_female_lice: x.avg_adult_female_lice.map(|y| (y * 100.0).round() / 100.0),
                avg_mobile_lice: x.avg_mobile_lice.map(|y| (y * 100.0).round() / 100.0),
                avg_stationary_lice: x.avg_stationary_lice.map(|y| (y * 100.0).round() / 100.0),
            })
            .collect();

        Self(data)
    }
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
    token_type: String,
    scope: String,
}

lazy_static! {
    static ref TOKEN_CACHE: Mutex<Option<(String, Instant)>> = Mutex::new(None);
}

pub struct Client {
    client: ReqwestClient,
    base_url: String,
    token_url: String,
    config: Option<ClientConfig>,
}

#[derive(Clone)]
pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: ReqwestClient::new(),
            base_url: "https://www.barentswatch.no/bwapi/v1/geodata".to_string(),
            token_url: "https://id.barentswatch.no/connect/token".to_string(),
            config: None,
        }
    }

    pub fn with_config(config: ClientConfig) -> Self {
        Self {
            client: ReqwestClient::new(),
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

        let mut form_params = HashMap::new();
        form_params.insert("client_id", config.client_id.clone());
        form_params.insert("scope", "api".to_string());
        form_params.insert("client_secret", config.client_secret.clone());
        form_params.insert("grant_type", "client_credentials".to_string());

        let res = self
            .client
            .post(&self.token_url)
            .form(&form_params)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await?;
            return Err(Error::Token(format!(
                "Token request failed: {} - {}",
                status, text
            )));
        }

        Ok(res.json::<TokenResponse>().await?)
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

        let res = self.client.get(url).bearer_auth(token).send().await?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await?;
            return Err(Error::Api(format!(
                "API request failed: {} - {}",
                status, text
            )));
        }

        Ok(res.json().await?)
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
            let mut res: Vec<BwApiApiVesselTrackWeeksModelsVesselTrackLocalityWeek> = self
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
