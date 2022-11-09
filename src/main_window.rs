use std::sync::mpsc::Receiver;
use std::time::Duration;

use eframe::egui::{self, CentralPanel};
use egui_extras::{Size, StripBuilder};

pub(crate) mod plot_pane;

mod log_settings_pane;
mod plot_settings_pane;
mod serial_settings_pane;

use self::log_settings_pane::LogSettingsPane;
use self::plot_pane::PlotPane;
use self::plot_settings_pane::PlotSettingsPane;
use self::serial_settings_pane::SerialSettingsPane;

pub struct MainWindow {
    pub plot_pane: PlotPane,
    pub plot_settings_pane: PlotSettingsPane,
    pub serial_settings_pane: SerialSettingsPane,
    pub log_settings_pane: LogSettingsPane,

    serial_data_rx: Receiver<String>,
}

impl MainWindow {
    pub fn new(serial_data_rx: Receiver<String>) -> Self {
        Self {
            plot_pane: PlotPane::new(),
            plot_settings_pane: PlotSettingsPane::new(),
            serial_settings_pane: SerialSettingsPane::new(),
            log_settings_pane: LogSettingsPane::new(),
            serial_data_rx,
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update the plot window with any new serial data.
        // Give it a timeout of zero so it is non-blocking.
        if let Ok(new_str) = self.serial_data_rx.recv_timeout(Duration::from_millis(0)) {
            self.plot_pane.lines.add_new_data(new_str.as_str())
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
                                    self.serial_settings_pane.update(ui);
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
        });

        ctx.request_repaint();
    }
}
