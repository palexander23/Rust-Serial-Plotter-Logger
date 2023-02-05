use eframe::egui;
use egui::plot::Plot;

use crate::multi_line::SerialDataMultiLine;

pub struct PlotPane {
    pub lines: SerialDataMultiLine,
}

impl PlotPane {
    pub fn new() -> Self {
        Self {
            lines: SerialDataMultiLine::new(),
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        // Plot the values stored in the local line storage
        // Process the stored plot values into lines to be plotted
        let plot_lines: Vec<egui::plot::Line> = self
            .lines
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
}
