use eframe::egui;

pub trait Component {
    fn new() -> Self
    where
        Self: Sized;

    fn update(&mut self, ctx: &egui::Context);
    fn draw(&self, ui: &mut egui::Ui, ctx: &egui::Context);
}
