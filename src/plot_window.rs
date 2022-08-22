use std::sync::{Arc, Mutex};

use eframe::egui;
use egui::plot::Plot;
use tracing::error;

use crate::multi_line::SerialDataMultiLine;

pub struct PlotWindow {
    pub lines: Arc<Mutex<SerialDataMultiLine>>,
}

impl PlotWindow {
    pub fn new() -> Self {
        Self {
            lines: Arc::new(Mutex::new(SerialDataMultiLine::new())),
        }
    }
}

impl eframe::App for PlotWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Serial Data");

            // Plot the values stored in the local line storage
            match self.lines.lock() {
                Ok(lines) => {
                    Plot::new("my_plot")
                        .show_axes([false, true])
                        .show(ui, |plot_ui| {
                            plot_ui.line(
                                egui::plot::Line::new(lines.line_vec[0].get_plot_values())
                                    .width(2.0),
                            );
                        });
                }
                Err(e) => error!("Could not get lock on line data: {:?}", e),
            };
        });

        ctx.request_repaint();
    }
}
