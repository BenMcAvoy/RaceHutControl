use crate::{component::Component, egui};

use super::all::Clock;

const HEADER: &str = "Race Hut Control";

pub struct Header {
    clock: Clock,
}

impl Component for Header {
    fn new() -> Self {
        Self {
            clock: Clock::new(),
        }
    }

    fn update(&mut self, ctx: &egui::Context) {
        self.clock.update(ctx);
    }

    fn draw(&self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading(HEADER);
            self.clock.draw(ui, ctx);
        });

        ui.separator();
    }
}

