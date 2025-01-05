use crate::{
    domain::{
        api::EndpointRequest,
        entity::{EntitiesEndpoint, EntitiesResponse, EntityEndpoint, EntityQuery, EntityResponse},
        license::{
            LicenseDetailResponse, LicenseEndpoint, LicenseQuery, LicenseSiteConnectionEndpoint,
            LicensesDetailResponse, LicensesEndpoint, SitesConnectionForLicenseResponse,
        },
        site::{
            EntityNumber, SiteEndpoint, SiteQuery, SiteResponse, SitesEndpoint,
            SitesForEntityEndpoint, SitesForEntityQuery, SitesForLegalEntityResponse,
            SitesResponse,
        },
    },
    error::Error,
};

const BASE_URL: &str = "https://api.fiskeridir.no/pub-aqua";

pub struct AkvaregRepository;

impl EndpointRequest for AkvaregRepository {
    fn base_url(&self) -> &str {
        BASE_URL
    }
}

impl AkvaregRepository {
    pub fn get_site(&self, id: i32) -> Result<SiteResponse, Error> {
        self.request::<SiteEndpoint>(&id)
    }
    pub fn get_sites(&self, query: Option<SiteQuery>) -> Result<SitesResponse, Error> {
        self.query_request::<SitesEndpoint>(&(), query)
    }
    pub fn get_sites_for_entity(
        &self,
        id: EntityNumber,
        query: Option<SitesForEntityQuery>,
    ) -> Result<SitesForLegalEntityResponse, Error> {
        self.query_request::<SitesForEntityEndpoint>(&id, query)
    }

    pub fn get_sites_for_license(
        &self,
        id: i32,
    ) -> Result<SitesConnectionForLicenseResponse, Error> {
        self.request::<LicenseSiteConnectionEndpoint>(&id)
    }

    pub fn get_license(&self, id: i32) -> Result<LicenseDetailResponse, Error> {
        self.request::<LicenseEndpoint>(&id)
    }
    pub fn get_licenses(
        &self,
        query: Option<LicenseQuery>,
    ) -> Result<LicensesDetailResponse, Error> {
        self.query_request::<LicensesEndpoint>(&(), query)
    }

    pub fn get_entity(&self, id: i32) -> Result<EntityResponse, Error> {
        self.request::<EntityEndpoint>(&id)
    }

    pub fn get_entities(&self, query: Option<EntityQuery>) -> Result<EntitiesResponse, Error> {
        self.query_request::<EntitiesEndpoint>(&(), query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{entity::EntityQueryBuilder, site::SitesForEntityQueryBuilder};

    #[test]
    fn test_get_entity() {
        let query = EntityQueryBuilder::default()
            .name("Lerøy".to_string())
            .build();

        let repo = AkvaregRepository;
        let entity = repo.get_entities(Some(query)).unwrap();
        for e in entity.0 {
            println!("{:?}", e);
        }
    }

    #[test]
    fn test_get_combined() {
        let query = SitesForEntityQueryBuilder::default()
            .activity_type(crate::domain::site::ActivityType::Commercial)
            .build();

        let entity_nr = EntityNumber::Organization(918904913);
        let repo = AkvaregRepository;
        let entity = repo.get_sites_for_entity(entity_nr, Some(query)).unwrap();
       
    }
}
