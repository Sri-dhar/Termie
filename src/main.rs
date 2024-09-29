use eframe::egui;
mod guii;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 1000.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Termie",
        options,
        Box::new(|_cc| Box::<guii::MyApp>::default()),
    )
}
