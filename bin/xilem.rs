use aqua_api_client::adapters::gui::xilem::run;
use xilem::EventLoop;

fn main() {
    dotenvy::dotenv().ok();
    run(EventLoop::with_user_event()).unwrap();
}