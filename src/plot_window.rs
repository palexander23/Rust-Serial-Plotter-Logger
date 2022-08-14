use std::sync::{Arc, Mutex};

use eframe::egui;
use egui::plot::{Line, Plot, Value, Values};

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
            Plot::new("my_plot").show(ui, |plot_ui| {
                plot_ui.line(egui::plot::Line::new(
                    self.line.lock().unwrap().get_plot_values(),
                ));
            });
        });

        ctx.request_repaint();
    }
}
