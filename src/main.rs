use eframe::egui;
use env_logger;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Yatskov Online",
        options,
        Box::new(|cc| Ok(Box::new(WebPage::new(cc)))),
    )
}

struct WebPage
{

}

impl WebPage {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for WebPage {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Yatskov Online");
            ui.add_space(4.0);

            ui.separator();
        });
    }
}
