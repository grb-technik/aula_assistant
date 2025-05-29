use crate::RuntimeOptions;

pub struct AppData {
    open_in_fullscreen: bool,
    hide_appbar: bool,
}

impl AppData {
    pub fn new(open_in_fullscreen: bool, hide_appbar: bool) -> Self {
        Self {
            open_in_fullscreen: open_in_fullscreen,
            hide_appbar: hide_appbar,
        }
    }

    pub fn is_open_in_fullscreen_set(&self) -> bool {
        self.open_in_fullscreen
    }

    pub fn is_hide_appbar_set(&self) -> bool {
        self.hide_appbar
    }
}

impl From<Vec<RuntimeOptions>> for AppData {
    fn from(options: Vec<RuntimeOptions>) -> Self {
        Self::new(
            options
                .iter()
                .any(|opt| matches!(opt, RuntimeOptions::OpenInFullscreen)),
            options
                .iter()
                .any(|opt| matches!(opt, RuntimeOptions::HideAppbar)),
        )
    }
}
