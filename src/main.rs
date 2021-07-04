mod common;
mod core;
mod fixtures;

use crate::common::{determine_which_os, get_apps};
use crate::core::App;
use crate::core::State;

fn main() {
    let which_os = determine_which_os();
    let mut app = App::new(which_os, State::new());
    let fixtures = fixtures::Applications::new(&app);

    for fixture_applications_folder in fixtures.folders {
        app.state.add_applications_folder(fixture_applications_folder);
    }

    let apps_list = get_apps(&app);
    println!("{:?}", apps_list);
}
