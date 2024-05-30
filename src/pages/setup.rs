use crate::components::all::*;
use crate::traits::{Component, ComponentV, Page};

use eframe::egui;

pub struct Setup {
    components: Vec<ComponentV>,
}

impl Page for Setup {
    fn new() -> Self {
        Self {
            components: vec![Box::new(Header::new()), Box::new(ClassList::new())],
        }
    }

    fn update(&mut self, ctx: &egui::Context) {
        for component in self.components.iter_mut() {
            component.update(ctx);
        }
    }

    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        for component in self.components.iter_mut() {
            component.draw(ui, ctx);
        }
    }
}
