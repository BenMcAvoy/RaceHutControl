use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // HACK: Fullscreen mode
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_fullscreen(true),

        ..Default::default()
    };

    eframe::run_simple_native("RaceHutControl", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.heading("Hello World!");
        });
    })
}
