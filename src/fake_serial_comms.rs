use std::num::Wrapping;

pub struct FakeSerialHandler {
    line_val_vec: Vec<Wrapping<i8>>,
    read_counter: u8,
}

impl FakeSerialHandler {
    pub fn new() -> Self {
        FakeSerialHandler {
            line_val_vec: vec![Wrapping(0), Wrapping(50), Wrapping(100), Wrapping(-50)],
            read_counter: 0,
        }
    }

    pub fn process_serial_data(&mut self) -> Option<String> {
        // If the line has been read recently don't give any more data
        if self.read_counter < 4 {
            self.read_counter += 1;
            return None;
        }

        // Format the stored vector of strings into a string
        let fake_serial_str: String = self
            .line_val_vec
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        // Reset the read counter for another delay between readings
        self.read_counter = 0;

        // Increment every line value
        self.line_val_vec = self.line_val_vec.iter().map(|v| v + Wrapping(1)).collect();

        // Return the fake serial string
        Some(fake_serial_str)
    }
}

pub fn get_available_port_names() -> Option<Vec<String>> {
    Some(vec!["Fake Port".to_string()])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fake_serial_comms() {
        let mut test_instance = FakeSerialHandler::new();

        assert_eq!(test_instance.process_serial_data(), None);
        assert_eq!(test_instance.process_serial_data(), None);
        assert_eq!(test_instance.process_serial_data(), None);
        assert_eq!(test_instance.process_serial_data(), None);

        assert_eq!(
            test_instance.process_serial_data().unwrap(),
            "0, 50, 100, -50"
        );

        assert_eq!(test_instance.process_serial_data(), None);
        assert_eq!(test_instance.process_serial_data(), None);
        assert_eq!(test_instance.process_serial_data(), None);
        assert_eq!(test_instance.process_serial_data(), None);

        assert_eq!(
            test_instance.process_serial_data().unwrap(),
            "1, 51, 101, -49"
        );
    }
}
