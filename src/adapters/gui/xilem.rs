use std::sync::Arc;

use winit::dpi::LogicalSize;
use winit::error::EventLoopError;
use winit::window::Window;
use xilem::core::{fork, frozen, MessageProxy, PhantomView};
use xilem::core::one_of::OneOf3;
use xilem::view::{
    button, flex, inline_prose, portal, prose, sized_box, spinner, worker, worker_raw, Axis, FlexExt, FlexSpacer, Padding
};
use xilem::{palette, EventLoopBuilder, TextAlignment, ViewCtx, WidgetView, Xilem};

use crate::adapters::repository::BarentsWatchRepository;
use crate::domain::barentswatch::combined::LiceTempTreatment;
use crate::domain::barentswatch::lice::LocalityYearParams;
use crate::domain::barentswatch::localities::{LocalitiesResponse, LocalitySearchQuery};
use crate::error::Error;

struct AquacultureData {
    localities: Vec<LocalityData>,
    selected_locality: Option<LiceTempTreatment>,
    api_send: std::sync::mpsc::SyncSender<ApiThreadMsg>,
    fetch_complete: FetchStatus,
    locality_to_fetch: Option<i32>,
}


#[derive(Debug, Clone, Copy, PartialEq)]
enum FetchStatus {
    NotStarted,
    InProgress,
    Error,
    Complete,
}

fn api_fetcher(data: &mut AquacultureData) -> impl PhantomView<AquacultureData, (), ViewCtx> {
    let sender = data.api_send.clone();

    worker_raw(
        (data.locality_to_fetch, data.fetch_complete),
        move |proxy, mut messages| {
            let sender = sender.clone();
            async move {
                while let Some((locality_to_fetch, status)) = messages.recv().await {
                    if !matches!(status, FetchStatus::InProgress) {
                        continue;
                    }
                    if locality_to_fetch.is_none() {
                        continue;
                    }
                    match sender.send(ApiThreadMsg::FetchLocalityData(LocalityYearParams { locality_no: locality_to_fetch.unwrap(), year: 2024 }, proxy.clone())) {
                        Ok(()) => {}
                        Err(_) => break,
                    };
                }
            }
        },
        |data: &mut AquacultureData, response: DataState| match response {
            DataState::LocalityAvailable(locality) => {
                data.selected_locality = Some(locality);
                data.fetch_complete = FetchStatus::Complete;
            },
            DataState::Error(err) => {
                println!("Error fetching data: {}", err);
                data.fetch_complete = FetchStatus::Error;
            },
            DataState::ListAvailable(localities) => {
                data.localities = localities.0.into_iter().map(|loc| LocalityData {
                    locality_no: loc.locality_no.unwrap(),
                    name: loc.name.unwrap(),
                    data_state: DataState::NotRequested,
                }).collect();
                data.fetch_complete = FetchStatus::Complete;
            },
            DataState::NotRequested => {}
            DataState::Pending => {}
        },
    )
}


#[derive(Debug)]
struct LocalityData {
    locality_no: i32,
    name: String,
    data_state: DataState,
}

#[derive(Debug, Clone)]
enum DataState {
    NotRequested,
    Pending,
    ListAvailable(LocalitiesResponse),
    LocalityAvailable(LiceTempTreatment),
    Error(String),
}



impl AquacultureData {
    fn view(&mut self) -> impl WidgetView<Self> {
        let left_column = Arc::new(sized_box(portal(flex((
            prose("Localities"),
            self.localities
                .iter_mut()
                .map(|loc| loc.list_view())
                .collect::<Vec<_>>()),
        )))
        .padding(Padding::leading(5.)));

        let (info_area, worker_value) =
            if let Some(selected_no) = self.selected_locality {
                if let Some(selected_locality) = self
                    .localities
                    .iter_mut()
                    .find(|it| it.locality_no == selected_no)
                {
                    // Trigger fetch if needed
                    let value = match selected_locality.data_state {
                        DataState::NotRequested => {
                            selected_locality.data_state = DataState::Pending;
                            Some(selected_no)
                        }
                        DataState::Pending => None,
                        DataState::Available(_) => None,
                        DataState::Error(_) => None,
                    };

                    (OneOf3::A(selected_locality.details_view()), value)
                } else {
                    (
                        OneOf3::B(
                            prose(format!("Locality {selected_no} selected, but not found."))
                                .alignment(TextAlignment::Middle)
                                .brush(palette::css::YELLOW),
                        ),
                        None,
                    )
                }
            } else {
                (OneOf3::C(
                prose("No selection yet made. Select a locality from the sidebar to continue.")
                    .alignment(TextAlignment::Middle),
            ), None)
            };

        fork(
            flex((
                FlexSpacer::Fixed(20.),
                flex((
                    left_column.flex(1.),
                    Arc::new(portal(sized_box(info_area).expand_width()).flex(1.)),
                ))
                .direction(Axis::Horizontal)
                .must_fill_major_axis(true)
                .flex(1.),
            ))
            .must_fill_major_axis(true),
            worker(
                worker_value,
                |proxy, mut rx| async move {
                    while let Some(request) = rx.recv().await {
                        if let Some(locality_no) = request {
                            let proxy = proxy.clone();

                            let repo =
                                BarentsWatchRepository::new().expect("Failed to create repository");

                            let params = LocalityYearParams {
                                locality_no,
                                year: 2024,
                            };

                            let result = create_combined_data(&repo, params);

                            let new_state = match result {
                                Ok(data) => DataState::Available(data),
                                Err(err) => DataState::Error(err.to_string()),
                            };

                            let _ = proxy.message((locality_no, new_state));
                        }
                    }
                },
                |state: &mut Self, (locality_no, new_state): (i32, DataState)| {
                    if let Some(locality) = state
                        .localities
                        .iter_mut()
                        .find(|l| l.locality_no == locality_no)
                    {
                        locality.data_state = new_state;
                    }
                },
            ),
        )
    }
}

impl LocalityData {
    fn list_view(&mut self) -> impl WidgetView<AquacultureData> {
        let locality_no = self.locality_no;
        flex((
            inline_prose(format!("{} - {}", self.locality_no, self.name)),
            button("Select", move |state: &mut AquacultureData| {
                state.selected_locality = Some(locality_no);
            }),
            FlexSpacer::Fixed(masonry::theme::SCROLLBAR_WIDTH),
        ))
        .direction(Axis::Horizontal)
    }

    fn details_view(&mut self) -> impl WidgetView<AquacultureData> {
        let content = match &self.data_state {
            DataState::NotRequested => OneOf3::A(
                prose("Failed to start fetching data. This is a bug!")
                    .alignment(TextAlignment::Middle),
            ),
            DataState::Pending => OneOf3::B(sized_box(spinner()).width(80.).height(80.)),
            DataState::Available(data) => {
                let header = sized_box(
                    flex((
                        sized_box(prose("Week")).width(50.),
                        sized_box(prose("Stationary")).width(60.),
                        sized_box(prose("Mature")).width(60.),
                        sized_box(prose("Mobile")).width(60.),
                        sized_box(prose("Sea Temperature")).width(60.),
                        sized_box(prose("Treatments")).width(300.),
                    ))
                    .direction(Axis::Horizontal),
                ).expand_width()
                .background(palette::css::LIGHT_GRAY);

                let mut weeks: Vec<i32> = data
                    .lice_distribution
                    .0
                    .iter()
                    .filter(|d| d.has_reported_lice)
                    .map(|d| d.week)
                    .collect();

                // Add weeks from temperature data
                weeks.extend(data.sea_temperature.0.data.iter().filter_map(|d| d.week));

                // Add weeks from treatments data
                weeks.extend(data.treatments.0.data.iter().filter_map(|d| d.week));

                // Deduplicate and sort weeks
                weeks.sort_unstable();
                weeks.dedup();

                let rows = weeks
                    .iter()
                    .map(|&week| {
                        let (stat, mat, mob) = data
                            .lice_distribution
                            .0
                            .iter()
                            .find(|d| d.week == week)
                            .map(|d| {
                                (
                                    d.avg_stationary_lice.unwrap_or_default(),
                                    d.avg_adult_female_lice.unwrap_or_default(),
                                    d.avg_mobile_lice.unwrap_or_default()
                                )
                                
                            })
                            .unwrap_or_else(|| (0., 0., 0.));

                     

                        let temp = data
                            .sea_temperature
                            .0
                            .data
                            .iter()
                            .find(|d| d.week == Some(week))
                            .map(|d| format!("{}°C", d.sea_temperature.unwrap_or_default()))
                            .unwrap_or_else(|| "-".to_string());

                        let treatment = data
                            .treatments
                            .0
                            .data
                            .iter()
                            .find(|d| d.week == Some(week))
                            .map(|d| format!("{:?}", d))
                            .unwrap_or_else(|| "-".to_string());

                        sized_box(
                            flex((
                                sized_box(prose(format!("{week}")).brush(palette::css::BLACK)).width(50.),
                                sized_box(prose(format!("{stat}")).brush(palette::css::BLACK)).width(60.),
                                sized_box(prose(format!("{mat}")).brush(palette::css::BLACK)).width(60.),
                                sized_box(prose(format!("{mob}")).brush(palette::css::BLACK)).width(60.),
                                sized_box(prose(temp).brush(palette::css::BLACK)).width(60.),
                                sized_box(prose(treatment).brush(palette::css::BLACK)).width(300.)
                            ))
                            .direction(Axis::Horizontal)
                        )
                        .expand_width()
                        .background(if week % 2 == 0 {
                            palette::css::WHITE
                        } else {
                            palette::css::LIGHT_GRAY
                        })
                    })
                    .collect::<Vec<_>>();

                OneOf3::C(
                    sized_box(flex((header, rows)).direction(Axis::Vertical))
                        .background(palette::css::WHITE)
                        
                )
            }
            DataState::Error(err) => OneOf3::A(
                prose(format!("Error fetching data: {}", err))
                    .alignment(TextAlignment::Middle)
                    .brush(palette::css::RED),
            ),
        };

        flex((
          
            prose(format!("Locality: {} - {}", self.locality_no, self.name))
                .alignment(TextAlignment::Middle),
            FlexSpacer::Fixed(1.),
            content,
           
        ))
        .main_axis_alignment(xilem::view::MainAxisAlignment::Start)
     
    }
}

pub fn run(event_loop: EventLoopBuilder) -> Result<(), EventLoopError> {
    let repo = BarentsWatchRepository::new().expect("Failed to initialize BarentsWatch repository");


    let (api_send, api_recv) = std::sync::mpsc::sync_channel::<ApiThreadMsg>(0);
    spawn_api_thread(api_recv).expect("Failed to spawn API thread");

    
    let localities = repo
        .fetch_localities("", true)
        .expect("Failed to fetch localities")
        .0
        .into_iter()
        .map(|loc| LocalityData {
            locality_no: loc.locality_no.unwrap(),
            name: loc.name.unwrap(),
            data_state: DataState::NotRequested,
        })
        .collect();

    let data = AquacultureData {
        localities,
        selected_locality: None,
        api_send,
    };

    let app = Xilem::new(data, AquacultureData::view);
    let min_window_size = LogicalSize::new(800., 600.);

    let window_attributes = Window::default_attributes()
        .with_title("Aquaculture Data")
        .with_resizable(true)
        .with_min_inner_size(min_window_size);

    app.run_windowed_in(event_loop, window_attributes)
}

fn create_combined_data(
    repo: &BarentsWatchRepository,
    params: LocalityYearParams,
) -> Result<LiceTempTreatment, Error> {
    let treatment = repo.fetch_locality_lice_treatments(&params);
    let temp = repo.fetch_locality_sea_temperature(params.clone());
    let lice = repo.fetch_locality_lice_distribution(&params);

    Ok(LiceTempTreatment {
        lice_distribution: lice?,
        sea_temperature: temp?,
        treatments: treatment?,
    })
}


enum ApiThreadMsg {
    FetchLocalityList(Option<LocalitySearchQuery>, MessageProxy<DataState>),
    FetchLocalityData(LocalityYearParams, MessageProxy<DataState>),
    
}



fn spawn_api_thread(
    api_recv: std::sync::mpsc::Receiver<ApiThreadMsg>,
) -> Result<(), Box<dyn std::error::Error>> {
    std::thread::Builder::new()
    .name(String::from("api_thread"))
    .spawn(move || {
        let api = BarentsWatchRepository::new().expect("Failed to create repository");
        for msg in api_recv {
            match msg {
                ApiThreadMsg::FetchLocalityList(search, proxy) => {
                    let _ = proxy.message(DataState::Pending);
                    let result = api.fetch_localities(search, true);
                    match result {
                        Ok(data) => {
                            let _ = proxy.message(DataState::ListAvailable(data));
                        }
                        Err(err) => {
                            let _ = proxy.message(DataState::Error(err.to_string()));
                        }
                    }
                }
                ApiThreadMsg::FetchLocalityData(params, proxy) => {
                    let _ = proxy.message(DataState::Pending);
                    let result = create_combined_data(&api, params);
                    match result {
                        Ok(data) => {
                            let _ = proxy.message(DataState::LocalityAvailable(data));
                        }
                        Err(err) => {
                            let _ = proxy.message(DataState::Error(err.to_string()));
                        }
                    }
                }
            }
        }
    })?;

    Ok(())

}