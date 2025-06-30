mod app;

use std::path::PathBuf;

use app::App;

pub struct DevCleanerGui {
    path: Option<PathBuf>,
    size: (f32, f32),
    title: String,
}

impl Default for DevCleanerGui {
    fn default() -> Self {
        Self {
            path: None,
            size: (600.0, 800.0),
            title: String::from("DevCleaner"),
        }
    }
}

impl DevCleanerGui {
    pub fn new(path: Option<PathBuf>) -> DevCleanerGui {
        DevCleanerGui {
            path,
            ..Default::default()
        }
    }

    pub fn with_window_size(self, size: (f32, f32)) -> Self {
        DevCleanerGui { size, ..self }
    }

    pub fn with_title(self, title: impl Into<String>) -> Self {
        DevCleanerGui {
            title: title.into(),
            ..self
        }
    }

    pub fn run(self) -> iced::Result {
        iced::daemon(|s: &App, _| s.title.clone(), App::update, App::view)
            .subscription(App::subscription)
            .run_with(move || App::new(self.title, self.path))
    }
}
