mod command;
mod file_operations;
mod gemini_integration;
mod ui;

use eframe::egui;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 1000.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Termie",
        options,
        Box::new(|_cc| Box::<ui::MyApp>::default()),
    )
}
