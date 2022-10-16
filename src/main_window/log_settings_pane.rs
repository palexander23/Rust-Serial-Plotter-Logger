use eframe::{
    egui::{self, Layout},
    emath::Align,
};

pub struct LogSettingsPane {
    text1: String,
    text2: String,
}

impl LogSettingsPane {
    pub fn new() -> Self {
        Self {
            text1: Default::default(),
            text2: Default::default(),
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                ui.heading("Log Settings");
                ui.horizontal(|ui| {
                    ui.label("thing");
                    ui.add(egui::TextEdit::singleline(&mut self.text1).hint_text("Text One"));
                });
                ui.horizontal(|ui| {
                    ui.label("thing2");
                    ui.add(egui::TextEdit::singleline(&mut self.text2).hint_text("Text Two"));
                });
            });
        });
    }
}
