pub(crate) mod consts;

use std::process::{Output, Command, Stdio};
use crate::App;
use std::collections::HashMap;
use crate::common::consts::{WINDOWS_TYPE, MACOS_TYPE, ERROR_COMMAND_OUTPUT_PANIC, WINDOWS_KEY_STR, ERROR_CHECK_OS, ERROR_CHECK_FOLDER_NOT_EXIST, LINUX_KEY_STR, LINUX_TYPE};
use std::path::Path;

pub fn determine_which_os() -> u16 {
    let os_name = std::env::consts::OS;
    let os_type = if os_name == WINDOWS_KEY_STR {
        WINDOWS_TYPE
    } else if os_name == LINUX_KEY_STR  {
        LINUX_TYPE
    } else {
        MACOS_TYPE
    };
    os_type
}

pub fn check_folder_exists(folder: &str) -> bool {
    Path::new(folder).is_dir()
}

pub fn do_ls(app: &App, workdir: &str) -> Result<Output, &'static str> {
    if check_folder_exists(workdir) == false {
        return Err(ERROR_CHECK_FOLDER_NOT_EXIST);
    }

    return match app.os_type {
        WINDOWS_TYPE => {
            let mut list_dir = Command::new("cmd");
            list_dir.args(&["/C", format!("cd /d {} && dir /s/b *.exe", workdir).as_str()]);
            let output = list_dir.output().expect("process failed to execute");
            Ok(output)
        },
        LINUX_TYPE => {
            let mut list_dir = Command::new("ls");
            list_dir.current_dir(workdir);
            let output = list_dir.output().expect("process failed to execute");
            Ok(output)
        },
        MACOS_TYPE => {
            let mut list_dir = Command::new("ls");
            list_dir.current_dir(workdir);
            let output = list_dir.output().expect("process failed to execute");
            Ok(output)
        }
        _ => {
            Err(ERROR_CHECK_OS)
        }
    };
}

pub fn get_apps(app: &App) -> Vec<HashMap<&'static str, Vec<String>>> {
    let apps_folders = app.state.get_applications_folders();
    let mut apps_list = Vec::new();

    let mut index = 0;
    loop {
        let current_folder = apps_folders[index];
        let ls_output = do_ls(&app, current_folder);

        match ls_output {
            Ok(ls_output_unwrap) => {
                match ls_output_unwrap {
                    Output { status, stderr, stdout } => {
                        if status.success() {
                            let buffer = stdout.to_vec();
                            let buffer_string = String::from_utf8(buffer).unwrap();

                            let apps_str_vec: Vec<String> = buffer_string.lines().map(|s| s.to_string()).collect();

                            let mut list = HashMap::new();
                            list.insert(current_folder, apps_str_vec);
                            apps_list.push(list);
                        } else {
                            let buffer = stderr.to_vec();
                            let buffer_string = String::from_utf8(buffer).unwrap();

                            panic!("{} - {:?}", ERROR_COMMAND_OUTPUT_PANIC, buffer_string)
                        }
                    }
                }
            }
            Err(ls_output_unwrap) => {
                panic!("{}", ls_output_unwrap);
            }
        }

        index += 1;
        if index >= apps_folders.len() {
            return apps_list;
        }
    }
}