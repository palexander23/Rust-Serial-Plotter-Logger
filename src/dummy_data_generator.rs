use rand::{thread_rng, Rng};
use std::collections::VecDeque;

pub type DataPoint = eframe::egui::plot::Value;

#[derive(Debug)]
struct SerialDataSingleLine {
    line: VecDeque<DataPoint>,
    x_counter: u64,

    random_range_max: u64,
    random_range_min: u64,
}

impl SerialDataSingleLine {
    // Generate a new line with an empty window
    pub fn new(rand_min: u64, rand_max: u64) -> Self {
        Self {
            line: VecDeque::new(),
            x_counter: 0,

            random_range_min: rand_min,
            random_range_max: rand_max,
        }
    }

    pub fn add_val(&mut self, val: i64) {
        // Construct a new x,y point.
        let new_point = DataPoint::new(self.x_counter as f64, val as f64);

        // Add the new value to the set of points.
        self.line.push_back(new_point);

        // Increment the x position
        self.x_counter += 1;
    }

    pub fn add_rand(&mut self) {
        // Generate new random value
        let mut rng = thread_rng();
        let new_val = rng.gen_range(self.random_range_min..=self.random_range_max) as i64;

        // Append the new value to the list of values
        self.add_val(new_val);
    }

    pub fn clear(&mut self) {
        self.line = VecDeque::new();
        self.x_counter = 0;
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_jnew_instance() {
        // Generate a new instance of the SerialDataSingleLine struct.
        let test_instance = SerialDataSingleLine::new(0, 10);

        assert_eq!(test_instance.x_counter, 0);
        assert_eq!(test_instance.line.into_iter().eq(vec![]), true);
    }

    #[test]
    fn test_add_value() {
        let mut test_instance = SerialDataSingleLine::new(0, 10);

        test_instance.add_val(5);
        test_instance.add_val(10);

        assert_eq!(
            test_instance.line,
            vec![DataPoint::new(0, 5), DataPoint::new(1, 10),]
        );

        assert_eq!(test_instance.x_counter, 2);
    }

    #[test]
    fn test_add_rand() {
        let mut test_instance = SerialDataSingleLine::new(0, 10);

        test_instance.add_rand();
        test_instance.add_rand();
        test_instance.add_rand();
        test_instance.add_rand();
        test_instance.add_rand();
        test_instance.add_rand();
        test_instance.add_rand();
        test_instance.add_rand();

        // Check values were added
        assert_eq!(test_instance.x_counter, 8);
        assert_eq!(test_instance.line.clone().into_iter().len(), 8);

        // Check added values fit within range
        // Below max
        assert_eq!(
            test_instance
                .line
                .clone()
                .into_iter()
                .map(|val| val.y)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
                <= test_instance.random_range_max as f64,
            true
        );

        // Below min
        assert_eq!(
            test_instance
                .line
                .clone()
                .into_iter()
                .map(|val| val.y)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
                <= test_instance.random_range_max as f64,
            true
        );

        println!("{:?}", test_instance);
    }

    #[test]
    fn test_clear() {
        let mut test_instance = SerialDataSingleLine::new(0, 10);

        test_instance.add_val(5);
        test_instance.add_val(10);
        assert_eq!(test_instance.x_counter, 2);

        test_instance.clear();
        assert_eq!(test_instance.x_counter, 0);

        assert_eq!(test_instance.line.into_iter().eq(vec![]), true);
    }
}
