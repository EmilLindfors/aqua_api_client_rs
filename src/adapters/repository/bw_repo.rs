use std::env;

use crate::{
    domain::{
        api::EndpointRequest,
        barentswatch::{
            api_config::{ClientConfig, JsonEndpoint, TokenResponse, TOKEN_CACHE}, lice::{
                LocalityLiceDistributionEndpoint, LocalityLiceDistributionResponse,
                LocalityLiceTreatmentsEndpoint, LocalityLiceTreatmentsResponse, LocalityYearParams,
            }, localities::{
                LocalitesEndpoint, LocalitiesResponse, LocalitySeaTemperatureEndpoint,
                LocalitySeaTemperatureResponse, LocalitySearchQuery,
            }, updated::{UpdateEndpoint, UpdateResponse}, vessel::{
                LocalityVesselVisitEndpoint, LocalityVesselVisits, VesselLocalityParams, Vessels,
                VesselsEndpoint,
            }
        },
    },
    error::Error,
};

const BW_BASE_URL: &str = "https://www.barentswatch.no/bwapi/v1/geodata";
const BW_TOKEN_URL: &str = "https://id.barentswatch.no/connect/token";

#[derive(Debug, Clone)]
pub struct BarentsWatchRepository {
    config: Option<ClientConfig>,
}

impl EndpointRequest for BarentsWatchRepository {
    fn base_url(&self) -> &str {
        BW_BASE_URL
    }
}

impl BarentsWatchRepository {
    pub fn new() -> Result<Self, Error> {
        let client_id = env::var("BARENTSWATCH_CLIENT_ID").map_err(|_| {
            Error::Environment(
                "BARENTSWATCH_CLIENT_ID environment variable must be set".to_string(),
            )
        })?;
        let client_secret = env::var("BARENTSWATCH_CLIENT_SECRET").map_err(|_| {
            Error::Environment(
                "BARENTSWATCH_CLIENT_SECRET environment variable must be set".to_string(),
            )
        })?;

        Ok(Self {
            config: Some(ClientConfig {
                client_id,
                client_secret,
            }),
        })
    }

    fn get_token(&self) -> Result<TokenResponse, crate::error::Error> {
        let mut cache = TOKEN_CACHE
            .lock()
            .map_err(|_| Error::Token("Failed to acquire token cache lock".to_string()))?;

        if let Some(token) = cache.as_ref() {
            if token.expires_in > chrono::Utc::now().timestamp() as u64 {
                return Ok(token.clone());
            }
        }

        let config = self
            .config
            .as_ref()
            .ok_or_else(|| Error::Auth("Config required for authentication".to_string()))?;

        let res = ureq::post(BW_TOKEN_URL).send_form(&[
            ("client_id", &config.client_id),
            ("scope", "api"),
            ("client_secret", &config.client_secret),
            ("grant_type", "client_credentials"),
        ])?;

        if !res.status() == 200 {
            let status = res.status();
            let text = res.into_string()?;
            return Err(Error::Token(format!(
                "Token request failed: {} - {}",
                status, text
            )));
        }

        let token = res.into_json::<TokenResponse>()?;
        *cache = Some(token.clone());
        Ok(token)
    }

    pub fn fetch_locality_vessel_visits(
        &self,
        params: VesselLocalityParams,
    ) -> Result<LocalityVesselVisits, Error> {
        let token = self.get_token()?;
        self.authenticated_request::<LocalityVesselVisitEndpoint>(
            &params,
            None,
            Some(token.access_token),
        )
    }

    pub fn fetch_update_status(
        &self,
        
    ) -> Result<UpdateResponse, Error> {
        let token = self.get_token()?;
        self.authenticated_request::<UpdateEndpoint>(
            &(),
            None,
            Some(token.access_token),
        )
    }


    pub fn fetch_vessels(&self) -> Result<Vessels, Error> {
        let token = self.get_token()?;
        self.authenticated_request::<VesselsEndpoint>(&(), None, Some(token.access_token))
    }
    /// Get information about a sites lice count for all different stages of lice development for a full year.
    pub fn fetch_locality_lice_distribution(
        &self,
        params: &LocalityYearParams,
    ) -> Result<LocalityLiceDistributionResponse, Error> {
        let token = self.get_token()?;
        self.authenticated_request::<LocalityLiceDistributionEndpoint>(
            params,
            None,
            Some(token.access_token),
        )
    }
    /// Get information about all treatments for a site for a full year, both in-feed and bath treatments.
    pub fn fetch_locality_lice_treatments(
        &self,
        params: &LocalityYearParams,
    ) -> Result<LocalityLiceTreatmentsResponse, Error> {
        let token = self.get_token()?;
        self.authenticated_request::<LocalityLiceTreatmentsEndpoint>(
            params,
            None,
            Some(token.access_token),
        )
    }

    /// Get information about localities, optionally filtered by only salmonids. Optionally search by name or id
    pub fn fetch_localities(
        &self,
        query: impl Into<LocalitySearchQuery>,
        only_salmonids: bool,
    ) -> Result<LocalitiesResponse, Error> {
        let token = self.get_token()?;
        self.authenticated_request::<LocalitesEndpoint>(
            &only_salmonids,
            Some(query.into()),
            Some(token.access_token),
        )
    }

    pub fn fetch_locality_sea_temperature(
        &self,
        params: LocalityYearParams,
    ) -> Result<LocalitySeaTemperatureResponse, Error> {
        let token = self.get_token()?;
        self.authenticated_request::<LocalitySeaTemperatureEndpoint>(
            &params,
            None,
            Some(token.access_token),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::barentswatch::combined::LiceTempTreatment;

    use super::*;

    #[test]
    fn test_fetch_localities() {
        dotenvy::dotenv().ok();
        let repo = BarentsWatchRepository::new().unwrap();
        let res = repo.fetch_localities("skorpo", false).unwrap();

        println!("{:?}", res);
    }

    #[test]
    fn test_fetch_update_status() {
        dotenvy::dotenv().ok();
        let repo = BarentsWatchRepository::new().unwrap();
        let res = repo.fetch_update_status().unwrap();

        println!("{}", res);
    }

    #[test]
    fn test_fetch_locality_sea_temperature() {
        dotenvy::dotenv().ok();
        let repo = BarentsWatchRepository::new().unwrap();
        let res = repo
            .fetch_locality_sea_temperature(LocalityYearParams {
                locality_no: 29936,
                year: 2023,
            })
            .unwrap();

        println!("{}", res);
    }

    #[test]
    fn test_fetch_locality_lice_distribution() {
        dotenvy::dotenv().ok();
        let repo = BarentsWatchRepository::new().unwrap();
        let res = repo
            .fetch_locality_lice_distribution(&LocalityYearParams {
                locality_no: 29936,
                year: 2023,
            })
            .unwrap();

        println!("{}", res);
    }

    #[test]
    fn test_fetch_locality_lice_treatments() {
        dotenvy::dotenv().ok();
        let repo = BarentsWatchRepository::new().unwrap();
        let res = repo
            .fetch_locality_lice_treatments(&LocalityYearParams {
                locality_no: 13035,
                year: 2024,
            })
            .unwrap();

        println!("{}", res);
    }

    #[test]
    fn test_fetch_locality_combined() {
        dotenvy::dotenv().ok();
        let repo = BarentsWatchRepository::new().unwrap();
        let treatment = repo
            .fetch_locality_lice_treatments(&LocalityYearParams {
                locality_no: 12839,
                year: 2024,
            })
            .unwrap();
        let temp = repo
            .fetch_locality_sea_temperature(LocalityYearParams {
                locality_no: 12839,
                year: 2024,
            })
            .unwrap();

        let lice = repo
            .fetch_locality_lice_distribution(&LocalityYearParams {
                locality_no: 12839,
                year: 2024,
            })
            .unwrap();

        let combined = LiceTempTreatment::new(lice, temp, treatment);

        println!("{}", combined);
    }
}
