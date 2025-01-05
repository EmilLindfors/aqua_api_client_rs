use crate::domain::generated::fish_health::BwApiApiLicereportModelsLiceReportMedicinalTreatmentType;

use super::{
    lice::{LocalityLiceDistributionResponse, LocalityLiceTreatmentsResponse},
    localities::LocalitySeaTemperatureResponse,
};
use tabled::{
    settings::{object::Segment, style::Style, Alignment, Modify, Width},
    Table, Tabled,
};

#[derive(Debug, Clone)]
pub struct LiceTempTreatment {
    pub lice_distribution: LocalityLiceDistributionResponse,
    pub sea_temperature: LocalitySeaTemperatureResponse,
    pub treatments: LocalityLiceTreatmentsResponse,
}

impl LiceTempTreatment {
    pub fn new(
        lice_distribution: LocalityLiceDistributionResponse,
        sea_temperature: LocalitySeaTemperatureResponse,
        treatments: LocalityLiceTreatmentsResponse,
    ) -> Self {
        LiceTempTreatment {
            lice_distribution,
            sea_temperature,
            treatments,
        }
    }
}

#[derive(Tabled)]
struct LiceDataRow {
    #[tabled(rename = "Week")]
    week: i32,
    #[tabled(rename = "Adult Female")]
    adult_female: String,
    #[tabled(rename = "Mobile")]
    mobile: String,
    #[tabled(rename = "Stationary")]
    stationary: String,
    #[tabled(rename = "Temp (°C)")]
    temperature: String,
    #[tabled(rename = "Treatments")]
    treatments: String,
}

impl std::fmt::Display for LiceTempTreatment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Get all weeks from all datasets
        let mut all_weeks: Vec<i32> = self
            .lice_distribution
            .0
            .iter()
            .filter(|d| d.has_reported_lice)
            .map(|d| d.week)
            .collect();

        // Add weeks from temperature data
        all_weeks.extend(self.sea_temperature.0.data.iter().filter_map(|d| d.week));

        // Add weeks from treatments data
        all_weeks.extend(self.treatments.0.data.iter().filter_map(|d| d.week));

        // Deduplicate and sort weeks
        all_weeks.sort_unstable();
        all_weeks.dedup();

        if all_weeks.is_empty() {
            return writeln!(f, "No data reported");
        }

        // Get metadata from first lice record if available
        if let Some(first) = self.lice_distribution.0.first() {
            writeln!(f, "Locality Lice, Temp and Treatment Data")?;
            writeln!(f, "Locality No: {}", first.locality_no)?;
            writeln!(f, "Year: {}", first.year)?;
            writeln!(f)?;
        }

        // Create table rows for each week
        let rows: Vec<LiceDataRow> = all_weeks
            .into_iter()
            .map(|week| {
                // Find lice data for this week
                let lice_data = self
                    .lice_distribution
                    .0
                    .iter()
                    .find(|d| d.has_reported_lice && d.week == week);

                // Find temperature data for this week
                let sea_temp = self
                    .sea_temperature
                    .0
                    .data
                    .iter()
                    .find(|d| d.week == Some(week));

                // Find treatments data for this week
                let treatments = self
                    .treatments
                    .0
                    .data
                    .iter()
                    .find(|d| d.week == Some(week))
                    .map(|treatment_record| {
                        let mut treatments = Vec::new();

                        for fish in &treatment_record.cleaner_fish_treatments {
                            treatments.push(format!("Cleaner Fish: {:?}", fish));
                        }

                        for combo in &treatment_record.combination_treatments {
                            let mut t = Vec::new();

                            for med in &combo.medicinal_treatments {
                                let mut treatments_str = String::from("");
                                treatments_str.push_str(&format!("{}", med.type_.unwrap_or(BwApiApiLicereportModelsLiceReportMedicinalTreatmentType::AnnenBehandling).to_string()));
                                if med.entire_locality.unwrap_or(false) {
                                    treatments_str.push_str(" (All Cages)");
                                } else {
                                    treatments_str.push_str(&format!(
                                        " (Cages: {})",
                                        med.number_of_cages.unwrap_or(-1)
                                    ));
                                }

                                t.push(treatments_str);
                            }

                                for non_med in &combo.non_medicinal_treatments {
                                    let mut treatments_str = String::from("");
                                    treatments_str.push_str(&format!("{}", non_med.type_.map_or("".to_string(), |x| x.to_string())));

                                    if non_med.entire_locality.unwrap_or(false) {
                                        treatments_str.push_str(" (All Cages)");
                                    } else {
                                        treatments_str.push_str(&format!(
                                            " (Cages: {})",
                                            non_med.number_of_cages.unwrap_or(-1)
                                        ));
                                    }

                                    t.push(treatments_str);

                                }

                                treatments.push(t.join(" + "));
                               
            


                        }

                        for med in &treatment_record.medicinal_treatments {
                            treatments.push(format!("Medicinal: {:?}", med));
                        }

                        for non_med in &treatment_record.non_medicinal_treatments {
                            treatments.push(format!(
                                "{} ({})",
                                non_med
                                    .type_
                                    .as_ref()
                                    .map_or("".to_string(), |x| x.to_string()),
                                non_med.entire_locality.map(|_x| "All Cages".to_string()).unwrap_or(non_med.number_of_cages.map(|x| format!("Cages: {}", x.to_string())).unwrap_or("".to_string()))
                            ));
                        }

                        treatments.join("\n")
                    })
                    .unwrap_or_default();

                LiceDataRow {
                    week,
                    adult_female: lice_data
                        .and_then(|d| d.avg_adult_female_lice)
                        .map(|v| format!("{:.2}", v))
                        .unwrap_or_else(|| "-".to_string()),
                    mobile: lice_data
                        .and_then(|d| d.avg_mobile_lice)
                        .map(|v| format!("{:.2}", v))
                        .unwrap_or_else(|| "-".to_string()),
                    stationary: lice_data
                        .and_then(|d| d.avg_stationary_lice)
                        .map(|v| format!("{:.2}", v))
                        .unwrap_or_else(|| "-".to_string()),
                    temperature: sea_temp
                        .and_then(|d| d.sea_temperature)
                        .map(|t| format!("{:.1}", t))
                        .unwrap_or_else(|| "-".to_string()),
                    treatments,
                }
            })
            .collect();

        // Create and format table
        let table = Table::new(rows)
            .with(Style::modern())
            .with(Modify::new(Segment::all()).with(Width::wrap(50))) // Allow wrapping for treatments
            .with(Alignment::right()) // Right-align numeric columns
            .to_string();

        write!(f, "{}", table)
    }
}

// Helper function to format Option<f64> values
fn format_option(value: Option<f64>) -> String {
    value
        .map(|v| format!("{:.2}", v))
        .unwrap_or_else(|| "-".to_string())
}
