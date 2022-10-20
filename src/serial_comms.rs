use serialport::SerialPort;
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
