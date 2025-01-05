use core::fmt;

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use tabled::{Table, Tabled};

use crate::{
    domain::api::{Endpoint, NoQuery},
    error::Error,
};

/// Get a timestamp for when the data sources was last imported. All other endpoint serve data last updated at these times.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct UpdateResponse {
    /// Timestamp for general aquaculture zone updates
    pub aquaculture: Option<DateTime<FixedOffset>>,

    /// Timestamp for cod spawning ground updates
    pub codspawningground: Option<DateTime<FixedOffset>>,

    /// Timestamp for farmed fish escape incident reports
    pub farmedfishescape: Option<DateTime<FixedOffset>>,

    /// Timestamp for Infectious Salmon Anemia (ISA) Pancreas Disease updates
    pub ilapd: Option<DateTime<FixedOffset>>,

    /// Timestamp for ISA protection zone updates
    pub ilaprotectionzone: Option<DateTime<FixedOffset>>,

    /// Timestamp for ISA surveillance zone updates
    pub ilasurveillancezone: Option<DateTime<FixedOffset>>,

    /// Timestamp for salmon river (laksevassdrag) updates
    pub laksevassdrag: Option<DateTime<FixedOffset>>,

    /// Timestamp for sea lice reporting updates
    pub licereport: Option<DateTime<FixedOffset>>,

    /// Timestamp for maritime boundary updates
    pub maritimeboundary: Option<DateTime<FixedOffset>>,

    /// Timestamp for Pancreas Disease protection zone updates
    pub pdprotectionzone: Option<DateTime<FixedOffset>>,

    /// Timestamp for Pancreas Disease surveillance zone updates
    pub pdsurveillancezone: Option<DateTime<FixedOffset>>,

    /// Timestamp for production area updates
    pub productionarea: Option<DateTime<FixedOffset>>,

    /// Timestamp for salmon fjord updates
    pub salmonfjord: Option<DateTime<FixedOffset>>,

    /// Timestamp for shrimp fishing ground updates
    pub shrimpfishingground: Option<DateTime<FixedOffset>>,

    /// Timestamp for slaughterhouse updates
    pub slaughterhouse: Option<DateTime<FixedOffset>>,
}

pub struct UpdateEndpoint;

impl Endpoint for UpdateEndpoint {
    type Query = NoQuery;
    type Response = UpdateResponse;
    type PathParams = ();

    fn path(_: &Self::PathParams) -> String {
        "/fishhealth/updatestatus".to_string()
    }
}

impl TryFrom<ureq::Response> for UpdateResponse {
    type Error = crate::error::Error;

    fn try_from(res: ureq::Response) -> Result<Self, Self::Error> {
        let status = res.status();
        if status == 200 {
            let json = res.into_json()?;
            Ok(json)
        } else {
            let text = res.into_string()?;
            Err(Error::Api(format!("Request failed: {} - {}", status, text)))
        }
    }
}

// Helper struct for table rows
#[derive(Tabled)]
struct UpdateRow {
    #[tabled(rename = "Endpoint Type")]
    name: String,
    #[tabled(rename = "Last Updated")]
    timestamp: String,
}

impl fmt::Display for UpdateResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = Vec::new();
        
        // Helper function to format DateTime nicely
        let format_date = |dt: &DateTime<FixedOffset>| {
            dt.format("%Y-%m-%d %H:%M").to_string()
        };

        // Helper to add row
        let add_row = |name: &str, opt_date:  &Option<DateTime<FixedOffset>>| {
            UpdateRow {
                name: name.to_string(),
                timestamp: opt_date
                    .as_ref()
                    .map(format_date)
                    .unwrap_or_else(|| "No data".to_string()),
            }
        };

        // Add all rows
        rows.push(add_row("Aquaculture", &self.aquaculture));
        rows.push(add_row("Cod Spawning Ground", &self.codspawningground));
        rows.push(add_row("Farmed Fish Escape", &self.farmedfishescape));
        rows.push(add_row("ISA PD", &self.ilapd));
        rows.push(add_row("ISA Protection Zone", &self.ilaprotectionzone));
        rows.push(add_row("ISA Surveillance Zone", &self.ilasurveillancezone));
        rows.push(add_row("Laksevassdrag", &self.laksevassdrag));
        rows.push(add_row("Lice Report", &self.licereport));
        rows.push(add_row("Maritime Boundary", &self.maritimeboundary));
        rows.push(add_row("PD Protection Zone", &self.pdprotectionzone));
        rows.push(add_row("PD Surveillance Zone", &self.pdsurveillancezone));
        rows.push(add_row("Production Area", &self.productionarea));
        rows.push(add_row("Salmon Fjord", &self.salmonfjord));
        rows.push(add_row("Shrimp Fishing Ground", &self.shrimpfishingground));
        rows.push(add_row("Slaughterhouse", &self.slaughterhouse));

        // Create and format table
        let table = Table::new(rows)
            .with(tabled::settings::style::Style::modern())
            .to_string();

        write!(f, "{}", table)
    }
}