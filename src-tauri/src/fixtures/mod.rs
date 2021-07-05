use crate::core::App;
use crate::common::consts::{WINDOWS_TYPE, MACOS_TYPE, LINUX_TYPE};

pub struct Applications {
    pub(crate) folders: Vec<&'static str>
}

impl Applications {
    pub(crate) fn new(app: &App) -> Self {
        if app.os_type == WINDOWS_TYPE {
            return Applications {
                folders: vec!["E:/Softwares"]
            };
        } else if app.os_type == LINUX_TYPE {
            return Applications {
                folders: vec!["/tmp"]
            };
        }

        Applications {
            folders: vec!["/Applications"]
        }
    }
}