use eframe::egui::{self, CentralPanel, Layout};
use eframe::emath::Align;

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
            ui.with_layout(Layout::top_down(Align::TOP), |ui| {
                ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                    self.serial_settings_pane.update(ui);
                    self.log_settings_pane.update(ui);
                });
                self.plot_settings_pane.update(ui);
                self.plot_pane.update(ui);
            });
        });

        ctx.request_repaint();
    }
}
