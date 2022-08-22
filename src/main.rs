#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::thread;
use std::time::Duration;

use tracing::{debug, error, info, warn, Level};

mod dummy_data_generator;
mod plot_window;
mod serial_comms;

fn main() {
    // Set up logging
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    info!("Starting app...");

    // Create an instance of the plot window
    let plot_win = plot_window::PlotWindow::new();

    // Extract a pointer to the line data storage object
    let line_data_ref = plot_win.line.clone();

    // Spin off a separate thread that will add new points to the line
    let mut serial_handler =
        serial_comms::SerialHandler::new("/dev/ttyACM0", serial_comms::Baud::BAUD9600);

    thread::spawn(move || loop {
        match serial_handler.process_serial_data() {
            Some(n) => line_data_ref.lock().unwrap().add_val(n),
            None => (),
        }
    });

    // Start the egui thread.
    // The program will not return from this!
    info!("Main thread started");
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Serial Plotter/Logger",
        native_options,
        Box::new(|_cc| Box::new(plot_win)),
    );
}
