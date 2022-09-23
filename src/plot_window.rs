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
                    // Process the stored plot values into lines to be plotted
                    let plot_lines: Vec<egui::plot::Line> = lines
                        .line_vec
                        .iter()
                        .map(|lv| lv.get_plot_values())
                        .map(|pv| egui::plot::Line::new(pv))
                        .map(|l| l.width(2.0))
                        .collect();

                    // Generate the plot window
                    Plot::new("my_plot")
                        .show_axes([false, true])
                        .show(ui, |plot_ui| {
                            plot_lines.into_iter().for_each(|l| plot_ui.line(l));
                        });
                }
                Err(e) => error!("Could not get lock on line data: {:?}", e),
            };
        });

        ctx.request_repaint();
    }
}
