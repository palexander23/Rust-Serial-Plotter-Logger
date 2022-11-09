use eframe::{
    egui::{self, Layout},
    emath::Align,
};
use strum::IntoEnumIterator;

use crate::baud::Baud;

#[cfg(feature = "fake-serial-comms")]
use crate::fake_serial_comms::get_available_port_names;
#[cfg(feature = "real-serial-comms")]
use crate::serial_comms::get_available_port_names;

#[derive(Default, Debug)]
pub struct SerialSettingsPaneInfo {
    selected_port_name: Option<String>,
    selected_baud_rate: Baud,
    start_clicked: bool,
    stop_clicked: bool,
}

pub struct SerialSettingsPane {
    selected_port_name_idx: usize,
    selected_baud_rate_idx: usize,
}

impl SerialSettingsPane {
    pub fn new() -> Self {
        Self {
            selected_port_name_idx: Default::default(),
            selected_baud_rate_idx: Default::default(),
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui) -> SerialSettingsPaneInfo {
        // Define an object in which to store in the useful data from this pane
        // Returned at the end of the update
        let mut pane_info = SerialSettingsPaneInfo::default();

        // Get a vec of strings representing the available serial ports.
        // If None is returned, create a Vec with single string stating "No Ports Available".
        // This will be shown in the ComboBox but None will be returned as the selected port.
        // TODO: Make the display of no available ports more elegant.
        let available_port_names: Vec<String>;
        let ports_available: bool;
        if let Some(port_names_vec) = get_available_port_names() {
            available_port_names = port_names_vec;
            ports_available = true;
        } else {
            available_port_names = vec!["No Ports Available".to_string()];
            ports_available = false;
        }

        // Get list of possible Baud Rates
        let baud_rates: Vec<_> = Baud::iter().collect();

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
                                &mut self.selected_port_name_idx,
                                available_port_names.len(),
                                |i| available_port_names[i].to_owned(),
                            );

                            ui.end_row();

                            egui::ComboBox::from_label("Baud Rate").show_index(
                                ui,
                                &mut self.selected_baud_rate_idx,
                                baud_rates.len(),
                                |i| baud_rates[i].into(),
                            );

                            ui.end_row();
                        });

                    // Collect and return the ComboBox settings
                    pane_info.selected_baud_rate = baud_rates[self.selected_baud_rate_idx];

                    if !ports_available {
                        pane_info.selected_port_name = None;
                    } else {
                        pane_info.selected_port_name =
                            Some(available_port_names[self.selected_port_name_idx].to_string());
                    }

                    ui.with_layout(
                        Layout::left_to_right(Align::Center).with_main_align(Align::Center),
                        |ui| {
                            pane_info.start_clicked = ui.button("Start").clicked();
                            pane_info.stop_clicked = ui.button("Stop").clicked();
                        },
                    );
                },
            );
        });

        return pane_info;
    }
}
