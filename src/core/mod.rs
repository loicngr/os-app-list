pub struct App {
    pub(crate) os_type: u8,
}

impl App {
    pub(crate) fn new(which_os: u8) -> Self {
        App { os_type: which_os }
    }
}