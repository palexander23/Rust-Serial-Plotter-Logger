#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::{thread, time::Duration};

use tracing::{debug, info, warn, Level};

mod baud;
mod main_window;
mod multi_line;
mod single_line;

use crate::main_window::gui_event_types::GuiEvent;

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
    let (gui_events_tx, gui_events_rx): (Sender<GuiEvent>, Receiver<GuiEvent>) = mpsc::channel();

    // Start Up the egui thread and backend thread.
    // The respective channels are passed in.
    run_backend_thread(serial_data_tx, gui_events_rx);
    run_egui_thread(serial_data_rx, gui_events_tx);
}

fn run_backend_thread(serial_data_tx: Sender<String>, gui_events_rx: Receiver<GuiEvent>) {
    // Spin the backend thread off on its own
    thread::spawn(move || loop {
        // Get a new gui event
        // THREAD BLOCKS HERE
        let new_gui_event = gui_events_rx.recv();

        if let Ok(GuiEvent::StartSerial(serial_settings)) = new_gui_event {
            debug!("Starting Serial Connection: {:?}", serial_settings);

            // Initialize the serial interface
            #[cfg(feature = "real-serial-comms")]
            let mut serial_handler = serial_comms::SerialHandler::new(
                &serial_settings.port_name,
                serial_settings.baud_rate,
            );

            #[cfg(feature = "fake-serial-comms")]
            let mut serial_handler = fake_serial_comms::FakeSerialHandler::new();

            loop {
                // If new serial data has arrived send it to the Guij
                if let Some(new_str) = serial_handler.process_serial_data() {
                    serial_data_tx.send(new_str.to_string()).unwrap();
                }

                // If the gui has been told to stop the serial port break the loop
                if let Ok(new_gui_event) = gui_events_rx.recv_timeout(Duration::from_millis(0)) {
                    if matches!(new_gui_event, GuiEvent::StopSerial) {
                        debug!("Serial connection halted");
                        break;
                    }
                }
            }
        }
    });
}

fn run_egui_thread(serial_data_rx: Receiver<String>, gui_events_tx: Sender<GuiEvent>) {
    // Create an instance of the plot window
    let main_win = main_window::MainWindow::new(serial_data_rx, gui_events_tx);

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
