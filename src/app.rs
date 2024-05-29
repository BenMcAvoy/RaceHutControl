use eframe::egui;

use crate::component::Component;
use crate::components::all::*;

pub struct App {
    components: Vec<Box<dyn Component>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            components: vec![Box::new(Header::new()), Box::new(ClassList::new())],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            for component in self.components.iter_mut() {
                component.update(ctx);
                component.draw(ui, ctx);
            }
        });
    }
}
