use aqua_api_client::adapters::gui::lice_app::IcedApp;

fn main() {
    dotenvy::dotenv().ok();
    IcedApp::run();
}