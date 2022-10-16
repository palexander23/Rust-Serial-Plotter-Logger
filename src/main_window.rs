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
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            plot_pane: PlotPane::new(),
            plot_settings_pane: PlotSettingsPane::new(),
            serial_settings_pane: SerialSettingsPane::new(),
            log_settings_pane: LogSettingsPane::new(),
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Define central display area for MainWindow
        CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::relative(0.1).at_least(80.0))
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
