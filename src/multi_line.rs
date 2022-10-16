use crate::single_line::SerialDataSingleLine;

pub struct SerialDataMultiLine {
    pub line_vec: Vec<SerialDataSingleLine>,
    line_count: usize,
    x_counter: i32,
    x_lookback_len: usize,
}

impl SerialDataMultiLine {
    pub fn new() -> Self {
        let line_vec = vec![SerialDataSingleLine::new()];

        Self {
            line_vec,
            line_count: 1,
            x_counter: 0,
            x_lookback_len: 30,
        }
    }

    pub fn add_new_data(&mut self, new_serial_line_str: &str) {
        // Get comma separated values from the new string
        let str_tokens: Vec<&str> = new_serial_line_str.trim().split(",").collect();

        // Add new lines to line_vec if the number of tokens is greater than the
        // number of line objects.
        let num_tokens = str_tokens.len();

        // Return if no new values were found to add to the line
        if str_tokens[0] == "" {
            return;
        }

        // If there are more tokens than lines to generate more lines
        while self.line_count < num_tokens {
            let new_line = SerialDataSingleLine::new();

            self.line_vec.push(new_line);
            self.line_count += 1;
        }

        // Trim each tocken and remove any empty tokens
        // TODO: Remove all non-numerical characters from each token
        let new_vals: Vec<i32> = str_tokens
            .iter()
            .map(|s| s.trim())
            .filter(|s| s != &"")
            .map(|s| s.parse().expect("Could not parse!"))
            .collect();

        // Place the parsed numbers into the respective line
        for (idx, val) in new_vals.iter().enumerate() {
            self.line_vec[idx].add_val(val.clone() as i64, self.x_counter);
        }

        // Prune the values in each line that have fallen behind the x look back.
        let x_cutoff = self.x_counter - self.x_lookback_len as i32;

        for idx in 0..self.line_count {
            self.line_vec[idx].prune_beyond_x_lookback(x_cutoff);
        }

        // Increment the x counter
        self.x_counter += 1;
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_new_multi_line_is_empty() {
        // Define a new object to run the tests
        let multiline_instance = SerialDataMultiLine::new();

        // Assert that a single line already exists
        assert!(multiline_instance.line_count == 1);

        // Assert that the line is empty
        let line_contents = multiline_instance.line_vec[0]._get_vec();
        assert_eq!(line_contents, vec![]);
    }

    #[test]
    fn test_add_value_to_first_line() {
        // Define a new object to run the tests
        let mut multiline_instance = SerialDataMultiLine::new();

        // Add a new value to the line
        multiline_instance.add_new_data("1\n\r");

        // Check that there is still only one line
        assert_eq!(multiline_instance.line_count, 1);

        // Check that the line has the new number in it
        let line_contents = multiline_instance.line_vec[0]._get_vec();
        assert_eq!(line_contents, vec![(0.0, 1.0)]);
    }

    #[test]
    fn test_create_second_line() {
        // Define a new object to run the tests
        let mut multiline_instance = SerialDataMultiLine::new();

        // Add a value to the first line
        multiline_instance.add_new_data("1\n\r");

        // Check that there is still only one line
        assert_eq!(multiline_instance.line_count, 1);

        // Send text with two values to process, forcing the creation of a second line
        multiline_instance.add_new_data("1, 2\n\r");

        // Check there are now two lines
        assert_eq!(multiline_instance.line_count, 2);

        // Get the two lines and check they have the correct values
        let line_0_vec = multiline_instance.line_vec[0]._get_vec();
        let line_1_vec = multiline_instance.line_vec[1]._get_vec();

        assert_eq!(line_0_vec, vec![(0.0, 1.0), (1.0, 1.0)]);
        assert_eq!(line_1_vec, vec![(1.0, 2.0)]);
    }

    #[test]
    fn test_add_three_lines_at_once() {
        // Define a new object to run the tests
        let mut multiline_instance = SerialDataMultiLine::new();

        // Add new data
        multiline_instance.add_new_data("1, 2, 3, 4\n\r");

        // Check the line count is correct
        assert_eq!(multiline_instance.line_count, 4);

        // Check the line values are correct
        let line_0_vec = multiline_instance.line_vec[0]._get_vec();
        let line_1_vec = multiline_instance.line_vec[1]._get_vec();
        let line_2_vec = multiline_instance.line_vec[2]._get_vec();
        let line_3_vec = multiline_instance.line_vec[3]._get_vec();

        assert_eq!(line_0_vec, vec![(0.0, 1.0)]);
        assert_eq!(line_1_vec, vec![(0.0, 2.0)]);
        assert_eq!(line_2_vec, vec![(0.0, 3.0)]);
        assert_eq!(line_3_vec, vec![(0.0, 4.0)]);
    }
}
