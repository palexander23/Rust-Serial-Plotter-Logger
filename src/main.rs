#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
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

    // Define channels for communication with the GUI thread
    let (serial_data_tx, serial_data_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // Start Up the egui thread and backend thread.
    // The respective channels are passed in.
    run_backend_thread(serial_data_tx);
    run_egui_thread(serial_data_rx);
}

fn run_backend_thread(serial_data_tx: Sender<String>) {
    // Initialize the serial interface
    #[cfg(feature = "real-serial-comms")]
    let mut serial_handler = serial_comms::SerialHandler::new("/dev/ttyACM0", Baud::BAUD9600);

    #[cfg(feature = "fake-serial-comms")]
    let mut serial_handler = fake_serial_comms::FakeSerialHandler::new();

    // Spin the backend thread off on its own
    thread::spawn(move || loop {
        if let Some(new_str) = serial_handler.process_serial_data() {
            serial_data_tx.send(new_str.to_string()).unwrap();
        }

        std::thread::sleep(Duration::from_millis(10));
    });
}

fn run_egui_thread(serial_data_rx: Receiver<String>) {
    // Create an instance of the plot window
    let main_win = main_window::MainWindow::new(serial_data_rx);

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
