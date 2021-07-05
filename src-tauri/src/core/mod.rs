pub struct App {
    pub(crate) os_type: u16,
    pub(crate) state: State
}

pub struct State {
    pub(crate) applications_folders: Vec<&'static str>
}

impl State {
    pub(crate) fn new() -> Self {
        State { applications_folders: vec![] }
    }

    pub(crate) fn add_applications_folder(&mut self, folder: &'static str) -> Vec<&'static str> {
        self.applications_folders.push(folder);
        self.applications_folders.clone()
    }

    pub(crate) fn get_applications_folders(&self) -> Vec<&'static str> {
        self.applications_folders.clone()
    }
}

impl App {
    pub(crate) fn new(which_os: u16, state: State) -> Self {
        App { os_type: which_os, state }
    }
}