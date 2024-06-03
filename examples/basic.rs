use egui::{Separator, Id};
use egui_editable_label::EditableLabel;

/// Create basic eframe app
fn main() {
    let _ = eframe::run_native(
        "Basic DragValueExpr Example",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(1024.0, 768.0)),
            ..Default::default()
        },
        Box::new(|_app| Box::<App>::default()),
    );
}

/// Simple data structure to hold mutable f64 value
struct App {
    value: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            value: "Hello, world!".to_string(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(EditableLabel::new(&mut self.value, Id::new("editable_label")));
            ui.add(Separator::default());
        });
    }
}
