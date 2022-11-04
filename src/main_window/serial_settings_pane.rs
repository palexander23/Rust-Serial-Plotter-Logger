use eframe::{
    egui::{self, Layout},
    emath::Align,
};
use serialport::{self, SerialPortInfo, SerialPortType};
use strum::IntoEnumIterator;

use crate::baud::Baud;

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

        // Get list of available serial port names
        let mut available_ports = serialport::available_ports().unwrap();

        // Sort port list numerically
        available_ports.sort_by_key(|p| p.port_name.replace("COM", "").parse::<i32>().unwrap());

        // Append device name to the combobox value
        let mut available_port_names: Vec<String> = available_ports
            .iter()
            .map(|p| (p.port_name.clone() + " " + &self.get_usb_device_name(p)).to_string())
            .collect();

        // Give a default empty value to the port names combo box
        if available_port_names.len() == 0 {
            available_port_names.push(String::from("No Ports Available"));
        }

        // Get list of possible Baud Rates
        let baud_rates: Vec<_> = Baud::iter().collect();

        ui.group(|ui| {
            ui.with_layout(
                Layout::top_down(Align::Center).with_cross_justify(true),
                |ui| {
                    ui.heading("Serial Settings");
                    egui::Grid::new("Serial Settings Grid")
                        .num_columns(2)
                        .show(ui, |ui| {
                            if available_port_names.len() > 0 {}
                            egui::ComboBox::from_label("Serial Port").show_index(
                                ui,
                                &mut self.selected_port_name_idx,
                                available_port_names.len(),
                                |i| available_port_names[i].to_owned(),
                            );

                            if available_ports.len() == 0 {
                                pane_info.selected_port_name = None;
                            } else {
                                pane_info.selected_port_name =
                                    Some(available_port_names[self.selected_port_name_idx].clone());
                            }

                            ui.end_row();

                            egui::ComboBox::from_label("Baud Rate").show_index(
                                ui,
                                &mut self.selected_baud_rate_idx,
                                baud_rates.len(),
                                |i| baud_rates[i].into(),
                            );

                            pane_info.selected_baud_rate = baud_rates[self.selected_baud_rate_idx];

                            ui.end_row();
                        });

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

    fn get_usb_device_name(&self, port_info: &SerialPortInfo) -> String {
        match &port_info.port_type {
            SerialPortType::UsbPort(usb_port_info) => match &usb_port_info.product {
                Some(product_name) => product_name.to_owned(),
                None => "".to_string(),
            },
            _ => "".to_string(),
        }
    }
}
