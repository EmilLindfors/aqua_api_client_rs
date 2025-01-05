use crate::domain::api::{Endpoint, NoQuery};
use crate::domain::barentswatch::vessel::*;
use crate::domain::generated::fish_health::*;
use crate::error::{Error, Result};
use lazy_static::lazy_static;

use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;
use std::fmt;
use std::num::TryFromIntError;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct LocalityLiceAndTreatmentData {
    locality_no: i32,
    year: i32,
    lice_data: LocalityLiceDistributionResponse,
    treatment_data: LocalityLiceTreatmentsResponse,
}

pub struct LocalityLiceTreatmentsEndpoint;

impl Endpoint for LocalityLiceTreatmentsEndpoint {
    type Query = NoQuery;
    type Response = LocalityLiceTreatmentsResponse;
    type PathParams = LocalityYearParams;

    fn path(params: &Self::PathParams) -> String {
        format!(
            "/fishhealth/locality/{}/liceTreatments/{}",
            params.locality_no, params.year
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct LocalityLiceTreatmentsResponse(
    pub BwApiApiFishhealthLocalityModelsAllTreatmentsGraphDataDto,
);

impl TryFrom<ureq::Response> for LocalityLiceTreatmentsResponse {
    type Error = crate::error::Error;

    fn try_from(response: ureq::Response) -> Result<Self> {
        if response.status() != 200 {
            return Err(Error::Api(response.into_string()?));
        }

        let res = response.into_json::<serde_json::Value>();

        match res {
            Ok(data) => {

                println!("{:?}", data);
            

                let mut d = Vec::new();

                for v in data["data"].as_array().unwrap() {
                    let cleaner_fish: Vec<BwApiApiFishhealthLocalityModelsCleanerFishTreatment> = v
                        ["cleanerFishTreatments"]
                        .as_array()
                        .map(|x|  x.iter()
                                .map(|y| {
                                    crate::domain::generated::fish_health::builder::BwApiApiFishhealthLocalityModelsCleanerFishTreatment::default()
                                      .entire_locality(y["entireLocality"].as_bool().unwrap())
                                      .name(y["name"].as_str().unwrap().to_string())
                                      .quantity (y["quantity"].as_i64().unwrap() as i32)
                                      .try_into().unwrap()
                                      
                                })
                                .collect())
                        .unwrap_or_default();

                    let combination: Vec<BwApiApiLicereportModelsLiceReportCombinationTreatment> = v
                        ["combinationTreatments"]
                        .as_array()
                        .map(|x|  x.iter()
                                .map(|y| {

                                    let medicinal_treatments: Vec<BwApiApiLicereportModelsLiceReportMedicinalTreatment> = 
    y["medicinalTreatments"]
        .as_array()
        .map(|treatments| {
            treatments
                .iter()
                .filter_map(|z| {
                    Some(crate::domain::generated::fish_health::builder::BwApiApiLicereportModelsLiceReportMedicinalTreatment::default()
                        .amount_unit(BwApiApiLicereportModelsLiceReportAmountUnit::try_from(
                            z["amountUnit"].as_str()?.to_string(),
                        ).ok()?)
                        .amount_value(z["amountValue"].as_f64()?)
                        .concentration_unit(BwApiApiLicereportModelsLiceReportConcentrationUnit::try_from(
                            z["concentrationUnit"].as_str()?.to_string(),
                        ).ok()?)
                        .concentration_value(z["concentrationValue"].as_f64()?)
                        .done_before_lice_count(z["doneBeforeLiceCount"].as_bool()?)
                        .entire_locality(z["entireLocality"].as_bool()?)
                        .id(z["id"].as_i64()? as i32)
                        .number_of_cages(z["numberOfCages"].as_i64()? as i32)
                        .other_substance(z["otherSubstance"].as_str()?.to_string())
                        .substance_id(z["substanceId"].as_i64()? as i32)
                        .type_(BwApiApiLicereportModelsLiceReportMedicinalTreatmentType::try_from(
                            z["type"].as_str()?.to_string(),
                        ).ok()?)
                        .try_into().ok()?)
                })
                .collect()
        })
        .unwrap_or_default();
                                    

                                    
        let non_medicinal_treatments: Vec<BwApiApiLicereportModelsLiceReportNonMedicinalTreatment> =  y["nonMedicinalTreatments"]
        .as_array()
        .map(|treatments| {
            treatments
                .iter()
                .filter_map(|z| {
                    Some(crate::domain::generated::fish_health::builder::BwApiApiLicereportModelsLiceReportNonMedicinalTreatment::default()
                       
                        .done_before_lice_count(z["doneBeforeLiceCount"].as_bool()?)
                        .entire_locality(z["entireLocality"].as_bool()?)
                        .id(z["id"].as_i64()?)
                        .number_of_cages(z["numberOfCages"].as_i64()? as i32)
                        .type_(BwApiApiLicereportModelsLiceReportNonMedicinalTreatmentType::try_from(
                            z["type"].as_str()?.to_string(),
                        ).ok()?)
                        .try_into().ok()?)
                })
                .collect()
        })
        .unwrap_or_default();

        crate::domain::generated::fish_health::builder::BwApiApiLicereportModelsLiceReportCombinationTreatment::default()
        .id(y["id"].as_i64().unwrap() as i32)
        .lice_report_id(y["liceReportId"].as_i64().unwrap())
        .medicinal_treatments(medicinal_treatments)
        .non_medicinal_treatments(non_medicinal_treatments)
        .try_into().unwrap()
                                })
                                .collect())
                        .unwrap_or_default();

                    let medicinal_treatments: Vec<BwApiApiFishhealthLocalityModelsMedicinalTreatmentDto> = 
                        v["medicinalTreatments"]
                            .as_array()
                            .map(|treatments| {
                                treatments
                                    .iter()
                                    .filter_map(|z| {
                                        Some(crate::domain::generated::fish_health::builder::BwApiApiFishhealthLocalityModelsMedicinalTreatmentDto::default()
                                            .entire_locality(z["entireLocality"].as_bool()?)
                                            .number_of_cages(z["numberOfCages"].as_i64()? as i32)
                                            .substance_id(z["substanceId"].as_i64()? as i32)
                                            .type_(z["type"].as_str()?.to_string())
                                            .try_into().ok()?)
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();


                            let non_medicinal_treatments: Vec<BwApiApiLicereportModelsLiceReportNonMedicinalTreatment> =  v["nonMedicinalTreatments"]
                            .as_array()
                            .map(|treatments| {
                                treatments
                                    .iter()
                                    .filter_map(|z| {
                                        Some(crate::domain::generated::fish_health::builder::BwApiApiLicereportModelsLiceReportNonMedicinalTreatment::default()
                                           
                                            .done_before_lice_count(z["doneBeforeLiceCount"].as_bool()?)
                                            .entire_locality(z["entireLocality"].as_bool()?)
                                            .id(z["id"].as_i64()?)
                                            .number_of_cages(z["numberOfCages"].as_i64()? as i32)
                                            .type_(BwApiApiLicereportModelsLiceReportNonMedicinalTreatmentType::try_from(
                                                z["type"].as_str()?.to_string(),
                                            ).ok()?)
                                            .try_into().ok()?)
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();

                    d.push(
                                crate::domain::generated::fish_health::builder::BwApiApiFishhealthLocalityModelsTreatmentGraphDataWeek::default()
                                    .week(v["week"].as_i64().unwrap() as i32)
                                    .version(v["version"].as_i64().unwrap() as i32)
                                    .cleaner_fish_treatments(cleaner_fish)
                                    .combination_treatments(combination)
                                    .medicinal_treatments(medicinal_treatments)
                                    .non_medicinal_treatments(non_medicinal_treatments)
                                    .try_into()
                                    .unwrap(),
                            );
                }

                let builder = crate::domain::generated::fish_health::builder::BwApiApiFishhealthLocalityModelsAllTreatmentsGraphDataDto::default().locality_no(data["localityNo"].as_i64().unwrap() as i32)
                .year(data["year"].as_i64().unwrap() as i32)
                .data(d)
                ;

                Ok(LocalityLiceTreatmentsResponse(
                    builder.try_into().unwrap(),
                ))
            }
            Err(e) => Err(Error::Api(e.to_string())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalityYearParams {
    pub locality_no: i32,
    pub year: i32,
}

pub struct LocalityLiceDistributionEndpoint;

impl Endpoint for LocalityLiceDistributionEndpoint {
    type Query = NoQuery;
    type Response = LocalityLiceDistributionResponse;
    type PathParams = LocalityYearParams;

    fn path(params: &Self::PathParams) -> String {
        format!(
            "/fishhealth/locality/{}/liceTypeDistribution/{}",
            params.locality_no, params.year
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiceDistribution {
    pub locality_no: i32,
    pub year: i32,
    pub week: i32,
    pub avg_adult_female_lice: Option<f64>,
    pub avg_mobile_lice: Option<f64>,
    pub avg_stationary_lice: Option<f64>,
    pub has_reported_lice: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LocalityLiceDistributionResponse(pub Vec<LiceDistribution>);

impl From<BwApiApiFishhealthLocalityModelsLocalityLiceDistributionGraphDataDto>
    for LocalityLiceDistributionResponse
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
                has_reported_lice: x.has_reported_lice.unwrap_or(false),
            })
            .collect();

        Self(data)
    }
}

impl TryFrom<ureq::Response> for LocalityLiceDistributionResponse {
    type Error = crate::error::Error;

    fn try_from(response: ureq::Response) -> Result<Self> {
        let dto: BwApiApiFishhealthLocalityModelsLocalityLiceDistributionGraphDataDto =
            response.into_json()?;
        Ok(dto.into())
    }
}

impl fmt::Display for LocalityLiceDistributionResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Get reported data only
        let reported_data: Vec<_> = self.0.iter().filter(|d| d.has_reported_lice).collect();

        // If no reported data, return early with a message
        if reported_data.is_empty() {
            return writeln!(f, "No lice distribution data reported");
        }

        // Get metadata from first record since it's same for all
        let first = &reported_data[0];
        writeln!(f, "Locality Lice Distribution Data")?;
        writeln!(f, "Locality No: {}", first.locality_no)?;
        writeln!(f, "Year: {}", first.year)?;
        writeln!(f)?;

        // Table header
        writeln!(f, "│ Week │ Adult Female │  Mobile  │ Stationary │")?;
        writeln!(f, "├──────┼──────────────┼──────────┼────────────┤")?;

        // Sort by week for consistent display
        let mut data = reported_data.clone();
        data.sort_by_key(|d| d.week);

        // Table rows
        for record in data.iter() {
            writeln!(
                f,
                "│ {:4} │ {:>12} │ {:>8} │ {:>10} │",
                record.week,
                format_option(record.avg_adult_female_lice),
                format_option(record.avg_mobile_lice),
                format_option(record.avg_stationary_lice),
            )?;
        }

        // Table footer
        writeln!(f, "└──────┴──────────────┴──────────┴────────────┘")?;

        // Statistics summary with peak weeks
        if let Some(stats) = calculate_statistics(&reported_data) {
            writeln!(f)?;
            writeln!(f, "Summary Statistics (Reported Weeks Only)")?;
            writeln!(
                f,
                "Adult Female - Mean: {}, Max: {} (Week {})",
                format_option(Some(stats.adult_female_mean)),
                format_option(Some(stats.adult_female_max)),
                stats
                    .adult_female_peak_week
                    .map_or("-".to_string(), |w| w.to_string())
            )?;
            writeln!(
                f,
                "Mobile      - Mean: {}, Max: {} (Week {})",
                format_option(Some(stats.mobile_mean)),
                format_option(Some(stats.mobile_max)),
                stats
                    .mobile_peak_week
                    .map_or("-".to_string(), |w| w.to_string())
            )?;
            writeln!(
                f,
                "Stationary  - Mean: {}, Max: {} (Week {})",
                format_option(Some(stats.stationary_mean)),
                format_option(Some(stats.stationary_max)),
                stats
                    .stationary_peak_week
                    .map_or("-".to_string(), |w| w.to_string())
            )?;
        }

        Ok(())
    }
}

// Helper function to format Option<f64> values
fn format_option(value: Option<f64>) -> String {
    value
        .map(|v| format!("{:.2}", v))
        .unwrap_or_else(|| "-".to_string())
}

// Statistics structure for summary
struct LiceStats {
    adult_female_mean: f64,
    adult_female_max: f64,
    adult_female_peak_week: Option<i32>,
    mobile_mean: f64,
    mobile_max: f64,
    mobile_peak_week: Option<i32>,
    stationary_mean: f64,
    stationary_max: f64,
    stationary_peak_week: Option<i32>,
}

// Helper function to calculate statistics
fn calculate_statistics(data: &[&LiceDistribution]) -> Option<LiceStats> {
    if data.is_empty() {
        return None;
    }

    let mut stats = LiceStats {
        adult_female_mean: 0.0,
        adult_female_max: f64::MIN,
        adult_female_peak_week: None,
        mobile_mean: 0.0,
        mobile_max: f64::MIN,
        mobile_peak_week: None,
        stationary_mean: 0.0,
        stationary_max: f64::MIN,
        stationary_peak_week: None,
    };

    let mut count_af = 0;
    let mut count_m = 0;
    let mut count_s = 0;

    for record in data {
        if let Some(val) = record.avg_adult_female_lice {
            stats.adult_female_mean += val;
            if val >= stats.adult_female_max {
                stats.adult_female_max = val;
                stats.adult_female_peak_week = Some(record.week);
            }
            count_af += 1;
        }
        if let Some(val) = record.avg_mobile_lice {
            stats.mobile_mean += val;
            if val >= stats.mobile_max {
                stats.mobile_max = val;
                stats.mobile_peak_week = Some(record.week);
            }
            count_m += 1;
        }
        if let Some(val) = record.avg_stationary_lice {
            stats.stationary_mean += val;
            if val >= stats.stationary_max {
                stats.stationary_max = val;
                stats.stationary_peak_week = Some(record.week);
            }
            count_s += 1;
        }
    }

    // Calculate means
    if count_af > 0 {
        stats.adult_female_mean /= count_af as f64;
    }
    if count_m > 0 {
        stats.mobile_mean /= count_m as f64;
    }
    if count_s > 0 {
        stats.stationary_mean /= count_s as f64;
    }

    Some(stats)
}

impl fmt::Display for LocalityLiceTreatmentsResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let data = self.0.clone();



        writeln!(f, "Locality Lice Treatment Data")?;
        writeln!(f, "Locality No: {}", data.locality_no.unwrap_or(-1))?;
        writeln!(f, "Year: {}", data.year.unwrap_or(-1))?;
        writeln!(f)?;

        // Table header
        writeln!(f, "│ Week │ Treatment │")?;
        writeln!(f, "├──────┼───────────┤")?;

        // Table rows
        for record in data.data.clone().into_iter() {
            let mut treatments_for_week = String::new();

            for cleaner_fish in record.cleaner_fish_treatments {
                treatments_for_week.push_str(&format!("Cleaner Fish: {:?}\n", cleaner_fish));
            }

            for mechanical in record.combination_treatments {
                treatments_for_week.push_str(&format!("Combination: {:?}\n", mechanical));
            }

            for medicinal in record.medicinal_treatments {
                treatments_for_week.push_str(&format!("Medicinal: {:?}\n", medicinal));
            }

            
            for non_medicinal in record.non_medicinal_treatments {
                treatments_for_week.push_str(&format!("{}, cages: {}\n", non_medicinal.type_.map_or("".to_string(), |x| x.to_string()), non_medicinal.number_of_cages.unwrap_or(-1)));
            }

            writeln!(
                f,
                "│ {:4} │ {}",
                record.week.unwrap_or(-1),
                treatments_for_week
            )?;
        }

        // Table footer
        writeln!(f, "└──────┴───────────┘")?;

        Ok(())
    }
}
