use eframe::{
    egui::{self, Layout},
    emath::Align,
};

pub struct SerialSettingsPane {
    text1: String,
    text2: String,
}

impl SerialSettingsPane {
    pub fn new() -> Self {
        Self {
            text1: Default::default(),
            text2: Default::default(),
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.with_layout(
                Layout::top_down(Align::LEFT).with_cross_justify(true),
                |ui| {
                    ui.heading("Serial Settings");
                    egui::Grid::new("Serial Settings Grid")
                        .num_columns(2)
                        .show(ui, |ui| {
                            ui.label("thing");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.text1).hint_text("Text One"),
                            );
                            ui.end_row();

                            ui.label("thing2");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.text2).hint_text("Text Two"),
                            );
                            ui.end_row();
                        });
                },
            );
        });
    }
}
