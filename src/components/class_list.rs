use crate::classes::*;
use crate::{component::Component, egui};

pub struct ClassList {
    pub classes: Vec<String>,

    selected: Class,
}

impl Component for ClassList {
    fn new() -> Self {
        Self {
            classes: Vec::new(),
            selected: Class::B420,
        }
    }

    fn update(&mut self, _ctx: &egui::Context) {}

    fn draw(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.heading("Class List");

        ui.horizontal(|ui| {
            egui::ComboBox::from_id_source("class_selector")
                .selected_text(format!("{:?}", self.selected))
                .show_ui(ui, |ui| {
                    for class in get_classes() {
                        ui.selectable_value(
                            &mut self.selected,
                            Class::from_str(class).unwrap(),
                            class,
                        );
                    }
                });

            let disabled = !self.classes.contains(&format!("{:?}", self.selected));
            if ui.add_enabled(disabled, egui::Button::new("Add")).clicked() {
                self.classes.push(format!("{:?}", self.selected.clone()));
            }
        });

        let classes_to_display = self.classes.clone();

        for class in &classes_to_display {
            ui.horizontal(|ui| {
                ui.label(class);

                if ui.button("Delete").clicked() {
                    let mut classes_clone = self.classes.clone();
                    classes_clone.retain(|c| c != class);
                    self.classes = classes_clone;
                }
            });
        }
    }
}
