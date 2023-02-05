use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use eframe::egui::{self, CentralPanel};
use egui_extras::{Size, StripBuilder};

pub(crate) mod plot_pane;

mod log_settings_pane;
mod plot_settings_pane;
mod serial_settings_pane;

use self::gui_event_types::{GuiEvent, SerialSettings};
use self::log_settings_pane::LogSettingsPane;
use self::plot_pane::PlotPane;
use self::plot_settings_pane::PlotSettingsPane;
use self::serial_settings_pane::{SerialSettingsPane, SerialSettingsPaneInfo};

pub mod gui_event_types {
    use crate::baud::Baud;

    #[derive(Debug)]
    pub enum GuiEvent {
        StopSerial,
        StartSerial(SerialSettings),
    }

    #[derive(Debug)]
    pub struct SerialSettings {
        pub port_name: String,
        pub baud_rate: Baud,
    }
}

pub struct MainWindow {
    pub plot_pane: PlotPane,
    pub plot_settings_pane: PlotSettingsPane,
    pub serial_settings_pane: SerialSettingsPane,
    pub log_settings_pane: LogSettingsPane,

    serial_data_rx: Receiver<String>,
    gui_events_tx: Sender<GuiEvent>,
}

impl MainWindow {
    pub fn new(serial_data_rx: Receiver<String>, gui_events_tx: Sender<GuiEvent>) -> Self {
        Self {
            plot_pane: PlotPane::new(),
            plot_settings_pane: PlotSettingsPane::new(),
            serial_settings_pane: SerialSettingsPane::new(),
            log_settings_pane: LogSettingsPane::new(),
            serial_data_rx,
            gui_events_tx,
        }
    }

    fn process_pane_info(&mut self, serial_settings_pane_info: SerialSettingsPaneInfo) {
        // If the start button has been pressed send a StartSerial event
        if serial_settings_pane_info.start_clicked {
            let new_start_event = GuiEvent::StartSerial(SerialSettings {
                port_name: serial_settings_pane_info.selected_port_name.unwrap(),
                baud_rate: serial_settings_pane_info.selected_baud_rate,
            });

            self.gui_events_tx.send(new_start_event).unwrap();
        }

        // If the stop button has been pressed send a StopSerial event
        if serial_settings_pane_info.stop_clicked {
            let new_stop_event = GuiEvent::StopSerial;

            self.gui_events_tx.send(new_stop_event).unwrap();
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update the plot window with any new serial data.
        // Give it a timeout of zero so it is non-blocking.
        // Loop until all data has been collected
        loop {
            if let Ok(new_str) = self.serial_data_rx.recv_timeout(Duration::from_millis(0)) {
                self.plot_pane.lines.add_new_data(new_str.as_str());
            } else {
                break;
            }
        }

        // Define central display area for MainWindow
        CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::relative(0.1).at_least(100.0))
                .size(Size::relative(0.1).at_least(80.0))
                .size(Size::remainder())
                .vertical(|mut strip| {
                    strip.strip(|builder| {
                        builder
                            .size(Size::relative(0.5).at_least(200.0))
                            .size(Size::remainder())
                            .horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    let serial_settings_pane_info =
                                        self.serial_settings_pane.update(ui);
                                    self.process_pane_info(serial_settings_pane_info);
                                });
                                strip.cell(|ui| {
                                    self.log_settings_pane.update(ui);
                                });
                            });
                    });

                    strip.strip(|builder| {
                        builder.size(Size::relative(1.0)).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                self.plot_settings_pane.update(ui);
                            });
                        });
                    });

                    strip.strip(|builder| {
                        builder.size(Size::relative(1.0)).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                self.plot_pane.update(ui);
                            });
                        });
                    });
                });

            ctx.request_repaint_after(Duration::from_millis(50))
        });
    }
}
