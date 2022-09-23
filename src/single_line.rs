use std::collections::VecDeque;
use tracing::debug;

use eframe::egui;

pub type DataPoint = eframe::egui::plot::Value;

#[derive(Debug)]
pub struct SerialDataSingleLine {
    line: VecDeque<DataPoint>,
}

impl SerialDataSingleLine {
    // Generate a new line with an empty window
    pub fn new() -> Self {
        Self {
            line: VecDeque::new(),
        }
    }

    pub fn add_val(&mut self, val: i64, x_counter: i32) {
        // Construct a new x,y point.
        let new_point = DataPoint::new(x_counter as f64, val as f64);

        // Add the new value to the set of points.
        self.line.push_back(new_point);
    }

    pub fn prune_beyond_x_lookback(&mut self, x_cutoff: i32) {
        if self.line.len() == 0 {
            return;
        }

        while self.line.front().expect("Could not get line front").x < x_cutoff as f64 {
            self.line.pop_front();

            if self.line.len() == 0 {
                return;
            }
        }
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        self.line = VecDeque::new();
    }

    pub fn get_plot_values(&self) -> egui::plot::Values {
        egui::plot::Values::from_values_iter(self.line.iter().copied())
    }

    pub fn get_vec(&self) -> Vec<(f64, f64)> {
        self.line.iter().map(|v| (v.x, v.y)).collect()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_jnew_instance() {
        // Generate a new instance of the SerialDataSingleLine struct.
        let test_instance = SerialDataSingleLine::new();

        assert_eq!(test_instance.line.into_iter().eq(vec![]), true);
    }

    #[test]
    fn test_add_value() {
        let mut test_instance = SerialDataSingleLine::new();

        test_instance.add_val(5, 0);
        test_instance.add_val(10, 1);

        assert_eq!(
            test_instance.line,
            vec![DataPoint::new(0, 5), DataPoint::new(1, 10),]
        );
    }

    #[test]
    fn test_clear() {
        let mut test_instance = SerialDataSingleLine::new();

        test_instance.add_val(5, 0);
        test_instance.add_val(10, 1);

        test_instance.clear();
        assert_eq!(test_instance.line.into_iter().eq(vec![]), true);
    }
}
