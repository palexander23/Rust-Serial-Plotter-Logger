use crate::single_line::SerialDataSingleLine;

pub struct SerialDataMultiLine {
    pub line_vec: Vec<SerialDataSingleLine>,
    line_count: usize,
}

impl SerialDataMultiLine {
    pub fn new() -> Self {
        let line_vec = vec![SerialDataSingleLine::new(0, 400)];

        Self {
            line_vec: line_vec,
            line_count: 1,
        }
    }

    pub fn add_new_data(&mut self, new_serial_line_str: &str) {
        // Get comma separated values from the new string
        let str_tokens: Vec<&str> = new_serial_line_str.trim().split(",").collect();

        // Add new lines to line_vec if the number of tokens is greater than the
        // number of line objects.
        let num_tokens = str_tokens.len();

        while self.line_count < num_tokens {
            let prev_line = &mut self.line_vec[self.line_count];

            let current_x = prev_line.x();
            let current_lookback_len = prev_line.x_lookback_length();

            let new_line = SerialDataSingleLine::new(current_x, current_lookback_len);
            self.line_vec.push(new_line);

            self.line_count += 1;
        }

        // Check if no new values were found
        if str_tokens[0] == "" {
            return;
        }

        let new_vals: Vec<i32> = str_tokens
            .iter()
            .map(|s| s.trim().parse().expect("Could not parse!"))
            .collect();

        for (idx, val) in new_vals.iter().enumerate() {
            self.line_vec[idx].add_val(val.clone() as i64);
        }
    }
}
