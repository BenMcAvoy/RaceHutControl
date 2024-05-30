use eframe::egui;

pub trait Component {
    fn new() -> Self
    where
        Self: Sized;

    fn update(&mut self, ctx: &egui::Context);
    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
}

pub trait Page {
    fn new() -> Self
    where
        Self: Sized;

    fn local_update(&mut self, _ctx: &egui::Context) {}

    fn update(&mut self, ctx: &egui::Context);
    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
}

pub type ComponentV = Box<dyn Component>;
pub type PageV = Box<dyn Page>;
