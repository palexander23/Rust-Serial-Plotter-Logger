use serialport::SerialPort;
use std::io::ErrorKind;
use std::time::Duration;
use tracing::{debug, error, info};

pub enum Baud {
    BAUD9600 = 9600,
    BAUD115200 = 115200,
}

impl Into<u32> for Baud {
    fn into(self) -> u32 {
        match self {
            Baud::BAUD9600 => 9600,
            Baud::BAUD115200 => 115_200,
        }
    }
}

pub struct SerialHandler<'a> {
    port_handle: Box<dyn serialport::SerialPort>,

    in_buffer: Box<Vec<char>>,

    deliminator: Box<&'a str>,
}

impl SerialHandler<'_> {
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
            deliminator: Box::new(""),
        }
    }

    pub fn process_serial_data(&mut self) -> Option<i64> {
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
        debug!("New Line Chars: {:?}", new_line_chars);

        let new_line_str: String = new_line_chars.iter().cloned().collect();
        debug!("New Line String: {:?}", new_line_str);

        self.in_buffer.drain(0..=line_ending_idx);

        // Attempt to convert value to number
        match new_line_str.trim().parse() {
            Ok(n) => {
                info!("Received number: {}", n);
                Some(n)
            }
            Err(_) => {
                info!("String Received: {}", new_line_str);
                return None;
            }
        }
    }
}
