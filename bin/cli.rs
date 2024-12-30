use aqua_api_client::{
    client::{Client, ClientConfig},
    error::Error
};
use clap::{Parser, Subcommand};
use csv::Writer;
use std::{env, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Locality {
        /// Locality ID
        locality_no: i32,
        /// Year
        year: i32,
        /// Week
        week: i32,
        /// Output to CSV file
        #[arg(long, value_name = "FILE")]
        csv: Option<PathBuf>,
    },
    LiceCount {
        /// Locality ID
        locality_no: i32,
        /// Year
        year: i32,
        /// Output to CSV file
        #[arg(long, value_name = "FILE")]
        csv: Option<PathBuf>,
    },
    LiceDistribution {
        /// Locality ID
        locality_no: i32,
        /// Year
        year: i32,
        /// Output to CSV file
        #[arg(long, value_name = "FILE")]
        csv: Option<PathBuf>,
    },
    VesselVisits {
        /// Locality ID
        locality_no: i32,
        /// Year
        year: i32,
        /// Week
        week: Option<i32>,
        /// Output to CSV file
        #[arg(long, value_name = "FILE")]
        csv: Option<PathBuf>,
    },
    Treatments {
        /// Locality ID
        locality_no: i32,
        /// Year
        year: i32,
        /// Output to CSV file
        #[arg(long, value_name = "FILE")]
        csv: Option<PathBuf>,
    },
    ListLocalities {
        /// Output to CSV file
        #[arg(long, value_name = "FILE")]
        csv: Option<PathBuf>,
    },
    ListSalmonidLocalities {
        /// Output to CSV file
        #[arg(long, value_name = "FILE")]
        csv: Option<PathBuf>,
    },
    /// Get production area by ID for specific week
    ProductionArea {
        /// Production area ID
        id: i32,
        /// Year
        year: i32,
        /// Week
        week: i32,
    },
    DiseaseZoneHistory {
        /// Locality ID
        locality_no: i32,
        /// Year
        year: Option<i32>,
        /// Week
        week: Option<i32>,
    },
    /// List all production areas
    ListProductionAreas,
    /// List PD zones
    ListPdZones,
     /// List all vessels
    ListVessels {
        /// Output to CSV file
        #[arg(long, value_name = "FILE")]
        csv: Option<PathBuf>,
    },
}

fn write_to_csv<T: serde::Serialize>(
    data: &[T],
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = Writer::from_path(path)?;
    for item in data {
        writer.serialize(item)?;
    }
    writer.flush()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    dotenvy::dotenv().ok();

    // Get auth from environment variables
    let client_id = env::var("BARENTSWATCH_CLIENT_ID").map_err(|_| {
        Error::Environment("BARENTSWATCH_CLIENT_ID environment variable must be set".to_string())
    })?;
    let client_secret = env::var("BARENTSWATCH_CLIENT_SECRET").map_err(|_| {
        Error::Environment(
            "BARENTSWATCH_CLIENT_SECRET environment variable must be set".to_string(),
        )
    })?;

    let client = Client::with_config(ClientConfig {
        client_id,
        client_secret,
    });

    match cli.command {
        Commands::Locality {
            locality_no,
            year,
            week,
            csv,
        } => {
            let report = client.get_locality_report(locality_no, year, week).await?;
            if let Some(csv_path) = csv {
                write_to_csv(&[report], &csv_path)?;
            } else {
                println!("Locality Report:");
                println!(
                    "  Locality: {} (ID: {})",
                    report.locality_name.unwrap_or_default(),
                    locality_no
                );
                println!(
                    "  Is in aquaculture register: {:?}",
                    report.locality_is_in_aqua_culture_register
                );
                if let Some(lice_count) = report.lice_count_previous_week {
                    println!("  Previous Week Lice Count:");
                    println!(
                        "    Adult Female Lice: {:?}",
                        lice_count.avg_adult_female_lice
                    );
                    println!("    Mobile Lice: {:?}", lice_count.avg_mobile_lice);
                    println!("    Stationary Lice: {:?}", lice_count.avg_stationary_lice);
                    println!("    Has Reported: {:?}", lice_count.has_reported_lice);
                }
                if let Some(week_data) = report.locality_week {
                    println!("  Current Week Data:");
                    println!("    Is Fallow: {:?}", week_data.is_fallow);
                    println!("    Has Salmonoids: {:?}", week_data.has_salmonoids);
                    println!("    Sea Temperature: {:?}", week_data.sea_temperature);
                }
            }
        }
        Commands::LiceCount {
            locality_no,
            year,
            csv,
        } => {
            let lice_data = client.get_locality_lice_count(locality_no, year).await?;
            if let Some(csv_path) = csv {
                write_to_csv(&lice_data.data, &csv_path)?;
            } else {
                println!("{:#?}", lice_data);
            }
        }
        Commands::LiceDistribution {
            locality_no,
            year,
            csv,
        } => {
            let lice_data = client
                .get_locality_lice_distribution(locality_no, year)
                .await?;
            if let Some(csv_path) = csv {
                write_to_csv(&lice_data.0, &csv_path)?;
            } else {
                println!("{:#?}", lice_data);
            }
        }
        Commands::VesselVisits {
            locality_no,
            year,
            week,
            csv,
        } => {
            let vessel_data = client
                .get_locality_vessel_visits(locality_no, year, week)
                .await?;
            if let Some(csv_path) = csv {
                write_to_csv(&vessel_data.0, &csv_path)?;
            } else {
                println!("{:#?}", vessel_data);
            }
        }
        Commands::Treatments {
            locality_no,
            year,
            csv,
        } => {
            let treatments = client.get_locality_treatments(locality_no, year).await?;
            if let Some(csv_path) = csv {
                write_to_csv(&treatments.data, &csv_path)?;
            } else {
                println!("{:#?}", treatments);
            }
        }
        Commands::DiseaseZoneHistory {
            locality_no,
            year,
            week,
        } => {
            let history = client
                .get_locality_diseasezonehistory_history(locality_no, year, week)
                .await?;

            println!("{:#?}", history);
        }
        Commands::ListLocalities { csv } => {
            let localities = client.get_localities().await?;
            if let Some(csv_path) = csv {
                write_to_csv(&localities, &csv_path)?;
            } else {
                for locality in localities {
                    println!(
                        "ID: {:?}, Name: {}, Municipality: {}",
                        locality.locality_no,
                        locality.name.unwrap_or_default(),
                        locality.municipality.unwrap_or_default()
                    );
                }
            }
        }
        Commands::ListSalmonidLocalities { csv } => {
            let localities = client.get_localities_with_salmonoids().await?;
            if let Some(csv_path) = csv {
                write_to_csv(&localities, &csv_path)?;
            } else {
                for locality in localities {
                    println!(
                        "ID: {:?}, Name: {}",
                        locality.locality_no,
                        locality.name.unwrap_or_default()
                    );
                }
            }
        }
        Commands::ProductionArea { id, year, week } => {
            let area = client.get_production_area(id, year, week).await?;
            println!("{:#?}", area);
        }
        Commands::ListProductionAreas => {
            let areas = client.get_production_areas().await?;
            for area in areas {
                println!("ID: {:?}, Name: {}", area.id, area.name.unwrap_or_default());
            }
        }
        Commands::ListPdZones => {
            let zones = client.get_pd_zones().await?;
            println!("{:#?}", zones);
        }
        Commands::ListVessels { csv } => {
            let vessels = client.get_vessels().await?;
            if let Some(csv_path) = csv {
                write_to_csv(&vessels.0, &csv_path)?;
            } else {
                for vessel in vessels.0 {
                    println!("{vessel}");
                }
            }
        }
    }

    Ok(())
}
