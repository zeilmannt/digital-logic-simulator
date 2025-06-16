mod ui;
mod circuit;
mod gate;
mod connection;

use ui::CircuitEditor;
use eframe::egui;


impl eframe::App for CircuitEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.draw(ctx);
    }
}


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Digital Logic Simulator",
        options,
        Box::new(|_cc| Box::new(CircuitEditor::new())),
    )
}


