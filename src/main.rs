mod common;
mod core;

use crate::common::{determine_which_os, get_apps};
use crate::core::App;

fn main() {
    let which_os = determine_which_os();
    let app = App::new(which_os);

    let apps_list = get_apps(&app);
    println!("{:?}", apps_list);
}
