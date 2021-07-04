use crate::core::App;
use crate::common::consts::{WINDOWS_TYPE, MACOS_TYPE};

pub struct Applications {
    pub(crate) folders: Vec<&'static str>
}

impl Applications {
    pub(crate) fn new(app: &App) -> Self {
        if app.os_type == WINDOWS_TYPE {
            return Applications {
                folders: vec!["E:/Softwares"]
            };
        }

        Applications {
            folders: vec!["/Applications"]
        }
    }
}