#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{thread, time::Duration};

use tracing::{info, warn, Level};

mod baud;
mod main_window;
mod multi_line;
mod single_line;

use baud::Baud;

#[cfg(feature = "real-serial-comms")]
mod serial_comms;

#[cfg(feature = "fake-serial-comms")]
mod fake_serial_comms;

fn main() {
    // Set up logging
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    info!("Starting app...");

    // Create an instance of the plot window
    let main_win = main_window::MainWindow::new();

    // Extract a pointer to the line data storage object
    let line_data_ref = main_win.plot_pane.lines.clone();

    // Spin off a separate thread that will add new points to the line
    #[cfg(feature = "real-serial-comms")]
    let mut serial_handler = serial_comms::SerialHandler::new("/dev/ttyACM0", Baud::BAUD9600);

    #[cfg(feature = "fake-serial-comms")]
    let mut serial_handler = fake_serial_comms::FakeSerialHandler::new();

    thread::spawn(move || loop {
        match serial_handler.process_serial_data() {
            Some(new_str) => line_data_ref.lock().unwrap().add_new_data(new_str.as_str()),
            None => (),
        }

        std::thread::sleep(Duration::from_millis(10));
    });

    // Start the egui thread.
    // The program will not return from this!
    info!("Main thread started");
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Serial Plotter/Logger",
        native_options,
        Box::new(|_cc| Box::new(main_win)),
    );
}
