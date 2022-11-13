use serialport::{SerialPort, SerialPortInfo, SerialPortType};
use std::io::ErrorKind;
use std::time::Duration;
use tracing::{debug, error};

use crate::baud::Baud;

pub struct SerialHandler {
    port_handle: Box<dyn serialport::SerialPort>,

    in_buffer: Box<Vec<char>>,
}

impl SerialHandler {
    pub fn new(port_name: &str, baud: Baud) -> Self {
        let new_port_handle = serialport::new(port_name.clone(), baud.into())
            .timeout(Duration::from_millis(1000))
            .open_native()
            .expect("Could not open serial port!");

        new_port_handle
            .clear(serialport::ClearBuffer::All)
            .expect("Could not clear serial buffers!");

        Self {
            port_handle: Box::new(new_port_handle),
            in_buffer: Box::new(vec![]),
        }
    }

    pub fn process_serial_data(&mut self) -> Option<String> {
        let mut read_buf: Vec<u8> = vec![0; 512];
        let mut bytes_read = 0;

        match self.port_handle.read(read_buf.as_mut_slice()) {
            Ok(t) => bytes_read = t,
            Err(ref e) if e.kind() == ErrorKind::TimedOut => return None,
            Err(e) => error!("Could not read serial! {:?}", e),
        }

        if bytes_read == 0 {
            return None;
        }

        let in_slice = &read_buf[0..bytes_read];
        let in_chars: Vec<char> = in_slice.into_iter().map(|b| b.clone().into()).collect();

        self.in_buffer.append(&mut in_chars.clone());

        if !in_chars.contains(&'\n') {
            return None;
        }

        let line_ending_idx = self
            .in_buffer
            .iter()
            .position(|&c| c == '\n')
            .expect("Could not find newline character!");

        let new_line_chars = &self.in_buffer[0..=line_ending_idx];
        let new_line_str: String = new_line_chars.iter().cloned().collect();
        debug!("New Line String: {:?}", new_line_str);

        self.in_buffer.drain(0..=line_ending_idx);

        Some(new_line_str)
    }
}

pub fn get_available_port_names() -> Option<(Vec<String>, Vec<String>)> {
    // Get list of available serial port names
    let mut available_ports = serialport::available_ports().unwrap();

    // If no ports exist, return here
    if available_ports.len() == 0 {
        return None;
    }

    // Sort port list numerically
    available_ports.sort_by_key(|p| p.port_name.replace("COM", "").parse::<i32>().unwrap());

    // Append device name to the combobox value
    let available_port_names: Vec<String> = available_ports
        .iter()
        .map(|p| p.port_name.clone())
        .collect();

    let available_port_details: Vec<String> = available_ports
        .iter()
        .map(|p| p.port_name.clone() + " " + &get_usb_device_name(p))
        .collect();

    return Some((available_port_names, available_port_details));
}

fn get_usb_device_name(port_info: &SerialPortInfo) -> String {
    match &port_info.port_type {
        SerialPortType::UsbPort(usb_port_info) => match &usb_port_info.product {
            Some(product_name) => product_name.to_owned(),
            None => "".to_string(),
        },
        _ => "".to_string(),
    }
}
