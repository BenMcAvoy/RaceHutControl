use eframe::{egui, AppCreator};

pub fn launch(title: &str, app: AppCreator) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1920.0, 1080.0])
            .with_fullscreen(true),
        ..Default::default()
    };

    eframe::run_native(title, options, app)
}

// fn main() -> Result<(), eframe::Error> {
//     let options = eframe::NativeOptions {
//         viewport: egui::ViewportBuilder::default()
//             .with_inner_size([1920.0, 1080.0])
//             .with_fullscreen(true),

//         ..Default::default()
//     };

//     eframe::run_native(
//         "RaceHutControl",
//         options,
//         Box::new(|_| Box::<App>::default()),
//     )
// }
