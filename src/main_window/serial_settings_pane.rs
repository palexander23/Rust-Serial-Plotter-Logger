use eframe::{
    egui::{self, Layout},
    emath::Align,
};
use serialport;
use strum::{EnumIter, IntoEnumIterator};

use crate::baud::{self, Baud};

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
        // Get list of available serial port names
        let available_port_names: Vec<String> = serialport::available_ports()
            .unwrap()
            .iter()
            .map(|s| s.port_name.clone())
            .collect();

        let mut selected_port_name_idx: usize = 0;

        // Get list of possible Baud Rates
        let baud_rates: Vec<_> = Baud::iter().collect();
        let mut selected_baud_rate_idx = 0;

        ui.group(|ui| {
            ui.with_layout(
                Layout::top_down(Align::LEFT).with_cross_justify(true),
                |ui| {
                    ui.heading("Serial Settings");
                    egui::Grid::new("Serial Settings Grid")
                        .num_columns(2)
                        .show(ui, |ui| {
                            egui::ComboBox::from_label("Serial Port").show_index(
                                ui,
                                &mut selected_port_name_idx,
                                available_port_names.len(),
                                |i| available_port_names[i].to_owned(),
                            );
                            ui.end_row();

                            egui::ComboBox::from_label("Baud Rate").show_index(
                                ui,
                                &mut selected_baud_rate_idx,
                                baud_rates.len(),
                                |i| baud_rates[i].into(),
                            );
                            ui.end_row();
                        });

                    ui.with_layout(
                        Layout::left_to_right(Align::Center).with_main_align(Align::Center),
                        |ui| {
                            ui.button("Start");
                            ui.button("Stop");
                        },
                    );
                },
            );
        });
    }
}
