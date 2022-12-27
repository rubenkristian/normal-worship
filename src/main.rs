use druid::{AppLauncher, WindowDesc};

mod data;
use data::AppState;

mod view;
use view::build_ui;

pub fn main() {
    let main_window = WindowDesc::new(build_ui()).title("Normal Worship");

    let initial_state = AppState {};

    AppLauncher::with_window(main_window).launch(initial_state).expect("Failed to launch application.");
}