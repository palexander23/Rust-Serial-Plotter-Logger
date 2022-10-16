use eframe::{
    egui::{self, Layout},
    emath::Align,
};

pub struct PlotSettingsPane {
    text1: String,
    text2: String,
}

impl PlotSettingsPane {
    pub fn new() -> Self {
        Self {
            text1: Default::default(),
            text2: Default::default(),
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                ui.heading("Plot Settings");
                egui::Grid::new("Plot Settings Grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("thing");
                        ui.add(egui::TextEdit::singleline(&mut self.text1).hint_text("Text One"));
                        ui.end_row();

                        ui.label("thing2");
                        ui.add(egui::TextEdit::singleline(&mut self.text2).hint_text("Text Two"));
                        ui.end_row();
                    });
            });
        });
    }
}
