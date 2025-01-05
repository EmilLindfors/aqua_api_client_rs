use iced::widget::{button, container, text};
use iced::{font, Color, Element, Font, Length, Task};

use crate::adapters::repository::BarentsWatchRepository;
use crate::domain::barentswatch::combined::LiceTempTreatment;
use crate::domain::barentswatch::lice::*;
use crate::domain::barentswatch::localities::LocalitySeaTemperatureResponse;
use crate::error::Error;

#[derive(Debug, Clone)]
pub enum Message {
    FetchCombinedData(i32, i32),  // locality_no, year
    LiceDataReceived(LocalityLiceDistributionResponse),
    TempDataReceived(LocalitySeaTemperatureResponse),
    TreatmentDataReceived(LocalityLiceTreatmentsResponse),
    Error(String),
    CombinedDataReady,
    FontLoaded(Result<(), font::Error>),
}

struct AppState {
    repository: BarentsWatchRepository,
    pending_data: PendingCombinedData,
    combined_data: Option<LiceTempTreatment>,
    error: Option<String>,
}

impl AppState {

    fn new() -> (Self, Task<Message>) {
        (
            AppState {
                repository: BarentsWatchRepository::new().unwrap(),
                pending_data: PendingCombinedData::default(),
                combined_data: None,
                error: None,
            },
            Task::batch([
                font::load(include_bytes!("../../../fonts/notosans-regular.ttf").as_slice())
                    .map(Message::FontLoaded),
                font::load(include_bytes!("../../../fonts/notosans-bold.ttf").as_slice())
                    .map(Message::FontLoaded),
            ]),
        )
    }
}

#[derive(Default)]
struct PendingCombinedData {
    lice: Option<LocalityLiceDistributionResponse>,
    temp: Option<LocalitySeaTemperatureResponse>,
    treatment: Option<LocalityLiceTreatmentsResponse>,
}

impl PendingCombinedData {
    fn try_combine(&self) -> Option<LiceTempTreatment> {
        match (&self.lice, &self.temp, &self.treatment) {
            (Some(lice), Some(temp), Some(treatment)) => {
                Some(LiceTempTreatment::new(
                    lice.clone(),
                    temp.clone(),
                    treatment.clone(),
                ))
            }
            _ => None,
        }
    }

    fn reset(&mut self) {
        self.lice = None;
        self.temp = None;
        self.treatment = None;
    }
}

impl AppState {

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FetchCombinedData(locality_no, year) => {
                self.pending_data.reset();
                self.error = None;
                
                let params = LocalityYearParams { locality_no, year };
                
                // Launch all three requests in parallel
                Task::batch(vec![
                    self.fetch_lice_data(params.clone()),
                    self.fetch_temperature_data(params.clone()),
                    self.fetch_treatment_data(params),
                ])
            }
            Message::LiceDataReceived(result) => {
         
                        self.pending_data.lice = Some(result);
                        if let Some(combined) = self.pending_data.try_combine() {
                            self.combined_data = Some(combined);
                            Task::perform(async {}, |_| Message::CombinedDataReady)
                        } else {
                            Task::none()
                        }
                    
                   
                
            }
            Message::TempDataReceived(result) => {
                
                        self.pending_data.temp = Some(result);
                        if let Some(combined) = self.pending_data.try_combine() {
                            self.combined_data = Some(combined);
                            Task::perform(async {}, |_| Message::CombinedDataReady)
                        } else {
                            Task::none()
                        }
                
                
            }
            Message::TreatmentDataReceived(result) => {
               
                        self.pending_data.treatment = Some(result);
                        if let Some(combined) = self.pending_data.try_combine() {
                            self.combined_data = Some(combined);
                            Task::perform(async {}, |_| Message::CombinedDataReady)
                        } else {
                            Task::none()
                        }
                    
                  
            }
            Message::FontLoaded(_) => Task::none(),
            Message::Error(e) => {
                self.error = Some(e);
                Task::none()
            }
            Message::CombinedDataReady => Task::none(),
        }
    }
    fn fetch_lice_data(&self, params: LocalityYearParams) -> Task<Message> {
        let repo = self.repository.clone();
        Task::perform(
            async move {
                std::thread::spawn(move || {
                    repo.fetch_locality_lice_distribution(&params)
                })
                .join()
                .unwrap_or(Err(Error::Internal("Thread panicked".into())))

                
            },
            |f| match f {
                Ok(res) => Message::LiceDataReceived(res),
                Err(e) => Message::Error(e.to_string()),
            }
        )
    }

    fn fetch_temperature_data(&self, params: LocalityYearParams) -> Task<Message> {
        let repo = self.repository.clone();
        Task::perform(
            async move {
                std::thread::spawn(move || {
                    repo.fetch_locality_sea_temperature(params)
                })
                .join()
                .unwrap_or(Err(Error::Internal("Thread panicked".into())))
            },
            |f| match f {
                Ok(res) => Message::TempDataReceived(res),
                Err(e) => Message::Error(e.to_string()),
            }
        )
    }

    fn fetch_treatment_data(&self, params: LocalityYearParams) -> Task<Message> {
        let repo = self.repository.clone();
        Task::perform(
            async move {
                std::thread::spawn(move || {
                    repo.fetch_locality_lice_treatments(&params)
                })
                .join()
                .unwrap_or(Err(Error::Internal("Thread panicked".into())))
            },
            |f| match f {
                Ok(res) => Message::TreatmentDataReceived(res),
                Err(e) => Message::Error(e.to_string()),
            }
        )
    }

    fn view(&self) -> Element<Message> {
        let content = iced::widget::column![
            button("Fetch Combined Data")
                .on_press(Message::FetchCombinedData(12839, 2024)),
            
            // Loading indicator
            if self.pending_data.try_combine().is_none() && self.error.is_none() {
                text("Loading...")
            } else {
                text("")
            },
            
            // Combined data display
            if let Some(combined) = &self.combined_data {
                text(format!("Combined Data: {}", combined))
            } else if self.error.is_none() {
                text("No data available")
            } else {
                text("")
            },
            
            // Error display
            if let Some(error) = &self.error {
                text(format!("Error: {}", error))
            } else {
                text("")
            }
        ];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
          
            .into()
    }
}

pub struct IcedApp; 


impl IcedApp {
    pub fn run() -> iced::Result {
        iced::application("LiceApp", AppState::update, AppState::view)
        .antialiasing(true)
        .default_font(Font::with_name("Noto Sans"))
      
        .run_with(AppState::new)
        
    }
}
