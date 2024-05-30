use crate::{egui, traits::Component};

use std::sync::{Arc, Mutex};

pub struct NextPage {
    pub pages_len: Option<usize>,
    pub page_id: Arc<Mutex<usize>>,
}

impl Component for NextPage {
    fn new() -> Self {
        Self {
            pages_len: None, // Should be set by the parent
            page_id: Arc::new(Mutex::new(0)),
        }
    }

    fn update(&mut self, _ctx: &egui::Context) {}

    fn draw(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::BOTTOM), |ui| {
            // let _ = ui.button("Next Page");
            if ui.button("Next Page").clicked() {
                let mut page_id = self.page_id.lock().unwrap();
                *page_id += 1;

                if let Some(pages_len) = self.pages_len {
                    if *page_id >= pages_len {
                        *page_id = 0;
                    }
                }
            }
        });
    }
}
