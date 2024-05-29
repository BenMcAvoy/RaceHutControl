use eframe::egui;

mod app;
mod component;
mod components;
mod window;

fn main() -> Result<(), eframe::Error> {
    window::launch("RaceHutControl", Box::new(|_| Box::<app::App>::default()))
}
