use crate::{egui, traits::Component};

pub struct Clock {
    datetime: String,
}

impl Component for Clock {
    fn new() -> Self {
        Self {
            datetime: String::from(""),
        }
    }

    fn update(&mut self, _ctx: &egui::Context) {
        self.datetime = chrono::Local::now().format("%e %B %Y @ %T").to_string();
    }

    fn draw(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.label(&self.datetime);
    }
}
