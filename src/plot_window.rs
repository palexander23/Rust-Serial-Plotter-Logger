use std::sync::{Arc, Mutex};

use eframe::egui;
use egui::plot::Plot;
use tracing::error;

use crate::dummy_data_generator::SerialDataSingleLine;

pub struct PlotWindow {
    pub line: Arc<Mutex<SerialDataSingleLine>>,
}

impl PlotWindow {
    pub fn new() -> Self {
        Self {
            line: Arc::new(Mutex::new(SerialDataSingleLine::new(0, 20))),
        }
    }
}

impl eframe::App for PlotWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Serial Data");

            // Plot the values stored in the local line storage
            match self.line.lock() {
                Ok(line_data) => {
                    Plot::new("my_plot").show(ui, |plot_ui| {
                        plot_ui.line(egui::plot::Line::new(line_data.get_plot_values()));
                    });
                }
                Err(_) => error!("Could not get lock on line data!"),
            };
        });

        ctx.request_repaint();
    }
}
