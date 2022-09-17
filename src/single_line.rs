use std::collections::VecDeque;

use eframe::egui;

pub type DataPoint = eframe::egui::plot::Value;

#[derive(Debug)]
pub struct SerialDataSingleLine {
    line: VecDeque<DataPoint>,
    x_counter: u32,
    x_lookback_len: usize,
}

impl SerialDataSingleLine {
    // Generate a new line with an empty window
    pub fn new(x_counter: u32, x_lookback_len: usize) -> Self {
        Self {
            line: VecDeque::new(),
            x_counter: x_counter,
            x_lookback_len: x_lookback_len,
        }
    }

    pub fn add_val(&mut self, val: i64) {
        // Construct a new x,y point.
        let new_point = DataPoint::new(self.x_counter as f64, val as f64);

        // Add the new value to the set of points.
        self.line.push_back(new_point);

        // Increment the x position
        self.x_counter += 1;

        // Pop off some old values if we need to
        while self.line.len() > self.x_lookback_len {
            self.line.pop_front();
        }
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        self.line = VecDeque::new();
        self.x_counter = 0;
    }

    pub fn get_plot_values(&self) -> egui::plot::Values {
        egui::plot::Values::from_values_iter(self.line.iter().copied())
    }

    pub fn get_vec(&self) -> Vec<f64> {
        self.line.iter().map(|v| v.y).collect()
    }

    pub fn set_lookback_length(&mut self, new_lookback_len: usize) {
        self.x_lookback_len = new_lookback_len
    }

    pub fn x_lookback_length(&mut self) -> usize {
        self.x_lookback_len
    }

    pub fn set_x(&mut self, new_x: u32) {
        self.x_counter = new_x;
    }

    pub fn x(&mut self) -> u32 {
        self.x_counter
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_jnew_instance() {
        // Generate a new instance of the SerialDataSingleLine struct.
        let test_instance = SerialDataSingleLine::new(0, 50);

        assert_eq!(test_instance.x_counter, 0);
        assert_eq!(test_instance.line.into_iter().eq(vec![]), true);
    }

    #[test]
    fn test_add_value() {
        let mut test_instance = SerialDataSingleLine::new(0, 50);

        test_instance.add_val(5);
        test_instance.add_val(10);

        assert_eq!(
            test_instance.line,
            vec![DataPoint::new(0, 5), DataPoint::new(1, 10),]
        );

        assert_eq!(test_instance.x_counter, 2);
    }

    #[test]
    fn test_clear() {
        let mut test_instance = SerialDataSingleLine::new(0, 50);

        test_instance.add_val(5);
        test_instance.add_val(10);
        assert_eq!(test_instance.x_counter, 2);

        test_instance.clear();
        assert_eq!(test_instance.x_counter, 0);

        assert_eq!(test_instance.line.into_iter().eq(vec![]), true);
    }
}
