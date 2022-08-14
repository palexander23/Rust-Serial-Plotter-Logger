#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod dummy_data_generator;
mod plot_window;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Serial Plotter",
        options,
        Box::new(|_cc| Box::new(plot_window::PlotWindow::default())),
    );
}
