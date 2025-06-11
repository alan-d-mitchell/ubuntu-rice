use eframe::egui;

struct ChessGUI;

impl eframe::App for ChessGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Chess");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Chess",
        options,
        Box::new(|_cc| Box::new(ChessGUI::default())),
    )
}