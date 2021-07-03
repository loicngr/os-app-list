mod consts;

use std::process::{Output, Command};
use crate::App;
use std::collections::HashMap;
use crate::common::consts::{WINDOWS_TYPE, MACOS_TYPE, ERROR_COMMAND_OUTPUT_PANIC, WINDOWS_KEY_STR};

pub const APPLICATIONS_FOLDERS: [&'static str; 1] = [
    "/Applications",
];

pub fn determine_which_os() -> u8 {
    let os_name = std::env::consts::OS;
    let os_type = if os_name == WINDOWS_KEY_STR {
        WINDOWS_TYPE
    } else {
        MACOS_TYPE
    };
    os_type
}

pub fn do_ls(app: &App, workdir: &str) -> Result<Output, ()> {
    match app.os_type {
        WINDOWS_TYPE => {},
        MACOS_TYPE => {
            let mut list_dir = Command::new("ls");
            list_dir.current_dir(workdir);
            let output = list_dir.output().expect("process failed to execute");
            return Ok(output);
        }
        _ => {
            return Err(());
        }
    }

    Err(())
}

pub fn get_apps_folders() -> [&'static str; 1] {
    APPLICATIONS_FOLDERS.clone()
}

pub fn get_apps(app: &App) -> Vec<HashMap<&'static str, Vec<String>>> {
    let apps_folders = get_apps_folders();
    let mut apps_list = Vec::new();

    let mut index = 0;
    loop {
        let current_folder = apps_folders[index];
        let ls_output = do_ls(&app, current_folder).unwrap();
        match ls_output {
            Output { status, stderr: _, stdout } => {
                if status.success() {
                    let buffer = stdout.to_vec();
                    let buffer_string = String::from_utf8(buffer).unwrap();

                    let apps_str_vec: Vec<String> = buffer_string.lines().map(|s| s.to_string()).collect();

                    let mut list = HashMap::new();
                    list.insert(current_folder, apps_str_vec);
                    apps_list.push(list);
                } else {
                    panic!("{}", ERROR_COMMAND_OUTPUT_PANIC)
                }
            }
        }

        index += 1;
        if index >= apps_folders.len() {
            return apps_list;
        }
    }
}