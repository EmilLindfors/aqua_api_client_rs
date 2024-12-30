use crate::fish_health::*;
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Vessel {
    is_slaughter_boat: ::std::option::Option<bool>,
    is_wellboat: ::std::option::Option<bool>,
    mmsi: ::std::option::Option<i32>,
    ship_register_vessel_type: ::std::option::Option<::std::string::String>,
    ship_register_vessel_type_name_en: ::std::option::Option<::std::string::String>,
    ship_register_vessel_type_name_no: ::std::option::Option<::std::string::String>,
    ship_type: ::std::option::Option<i32>,
    vessel_name: ::std::option::Option<::std::string::String>,
}

impl fmt::Display for Vessel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        res.push_str(&format!("Vessel name: {}\n", self.vessel_name.as_ref().unwrap_or(&"".to_string())));
        res.push_str(&format!("MMSI: {}\n", self.mmsi.unwrap_or(0)));
        res.push_str(&format!("Ship type: {}\n", self.ship_type.unwrap_or(0)));
        res.push_str(&format!("Ship register vessel type: {}\n", self.ship_register_vessel_type.as_ref().unwrap_or(&"".to_string())));
        res.push_str(&format!("Ship register vessel type name (EN): {}\n", self.ship_register_vessel_type_name_en.as_ref().unwrap_or(&"".to_string())));
        res.push_str(&format!("Ship register vessel type name (NO): {}\n", self.ship_register_vessel_type_name_no.as_ref().unwrap_or(&"".to_string())));
        res.push_str(&format!("Is slaughter boat: {}\n", self.is_slaughter_boat.unwrap_or(false)));
        res.push_str(&format!("Is wellboat: {}\n", self.is_wellboat.unwrap_or(false)));

        write!(f, "{}", res)

    }
}

impl From<BwApiApiVesselTrackWeeksModelsVesselInfo> for Vessel {
    fn from(dto: BwApiApiVesselTrackWeeksModelsVesselInfo) -> Self {
        Self {
            is_slaughter_boat: dto.is_slaughter_boat,
            is_wellboat: dto.is_wellboat,
            mmsi: dto.mmsi,
            ship_register_vessel_type: dto.ship_register_vessel_type.clone(),
            ship_register_vessel_type_name_en: dto.ship_register_vessel_type_name_en.clone(),
            ship_register_vessel_type_name_no: dto.ship_register_vessel_type_name_no.clone(),
            ship_type: dto.ship_type,
            vessel_name: dto.vessel_name.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Vessels(pub Vec<Vessel>);

impl From<Vec<BwApiApiVesselTrackWeeksModelsVesselInfo>> for Vessels {
    fn from(dto: Vec<BwApiApiVesselTrackWeeksModelsVesselInfo>) -> Self {
        let data = dto
            .into_iter()
            .map(|x| Vessel {
                is_slaughter_boat: x.is_slaughter_boat,
                is_wellboat: x.is_wellboat,
                mmsi: x.mmsi,
                ship_register_vessel_type: x.ship_register_vessel_type.clone(),
                ship_register_vessel_type_name_en: x.ship_register_vessel_type_name_en.clone(),
                ship_register_vessel_type_name_no: x.ship_register_vessel_type_name_no.clone(),
                ship_type: x.ship_type,
                vessel_name: x.vessel_name.clone(),
            })
            .collect();

        Self(data)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VesselVisit {
    locality_no: i32,
    year: i32,
    week: Option<i32>,
    is_slaughter_boat: ::std::option::Option<bool>,
    is_wellboat: ::std::option::Option<bool>,
    mmsi: ::std::option::Option<i32>,
    ship_register_vessel_type: ::std::option::Option<::std::string::String>,
    ship_register_vessel_type_name_en: ::std::option::Option<::std::string::String>,
    ship_register_vessel_type_name_no: ::std::option::Option<::std::string::String>,
    ship_type: ::std::option::Option<i32>,
    start_time: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
    stop_time: ::std::option::Option<chrono::DateTime<chrono::offset::Utc>>,
    vessel_name: ::std::option::Option<::std::string::String>,
}

#[derive(Debug, Deserialize)]
pub struct LocalityVesselVisits(pub Vec<VesselVisit>);

impl From<BwApiApiVesselTrackWeeksModelsVesselTrackLocalityWeek> for LocalityVesselVisits {
    fn from(dto: BwApiApiVesselTrackWeeksModelsVesselTrackLocalityWeek) -> Self {
        let data = dto
            .vessel_visits
            .into_iter()
            .map(|x| VesselVisit {
                locality_no: dto.locality_no.unwrap_or(0),
                year: dto.year.unwrap_or(0),
                week: dto.week,
                is_slaughter_boat: x.is_slaughter_boat,
                is_wellboat: x.is_wellboat,
                mmsi: x.mmsi,
                ship_register_vessel_type: x.ship_register_vessel_type.clone(),
                ship_register_vessel_type_name_en: x.ship_register_vessel_type_name_en.clone(),
                ship_register_vessel_type_name_no: x.ship_register_vessel_type_name_no.clone(),
                ship_type: x.ship_type,
                start_time: x.start_time,
                stop_time: x.stop_time,
                vessel_name: x.vessel_name.clone(),
            })
            .collect();

        Self(data)
    }
}

impl From<Vec<BwApiApiVesselTrackWeeksModelsVesselTrackLocalityWeek>> for LocalityVesselVisits {
    fn from(dto: Vec<BwApiApiVesselTrackWeeksModelsVesselTrackLocalityWeek>) -> Self {
        let mut res = Vec::new();

        for item in dto {
            for visit in item.vessel_visits {
                res.push(VesselVisit {
                    locality_no: item.locality_no.unwrap_or(0),
                    year: item.year.unwrap_or(0),
                    week: item.week,
                    is_slaughter_boat: visit.is_slaughter_boat,
                    is_wellboat: visit.is_wellboat,
                    mmsi: visit.mmsi,
                    ship_register_vessel_type: visit.ship_register_vessel_type.clone(),
                    ship_register_vessel_type_name_en: visit
                        .ship_register_vessel_type_name_en
                        .clone(),
                    ship_register_vessel_type_name_no: visit
                        .ship_register_vessel_type_name_no
                        .clone(),
                    ship_type: visit.ship_type,
                    start_time: visit.start_time,
                    stop_time: visit.stop_time,
                    vessel_name: visit.vessel_name.clone(),
                });
            }
        }

        Self(res)
    }
}