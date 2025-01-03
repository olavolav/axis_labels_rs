use chrono::DateTime;
use chrono::Local;

mod datetime_range;
mod float_range;
use datetime_range::datetime_axis_labels;
use float_range::float_axis_labels;

pub struct AxisLabels<T: PartialOrd> {
    x_min: T,
    x_max: T,
    available_space: u32,
    padding_left: u32,
    vertical_direction: bool,
    unit: String,
}

impl<T: PartialOrd> AxisLabels<T> {
    pub fn new(x_min: T, x_max: T, available_space: u32, vertical_direction: bool) -> Self {
        // Check arguments
        if available_space == 0 {
            panic!("Invalid arguments: available_space == 0.");
        }
        if x_max < x_min {
            panic!("Invalid arguments: x_max < x_min.");
        }

        return Self {
            x_min,
            x_max,
            available_space,
            padding_left: 0,
            vertical_direction,
            unit: String::from(""),
        };
    }

    pub fn with_unit(&mut self, unit: String) {
        self.unit = unit;
    }

    pub fn with_padding_left(&mut self, padding_left: u32) {
        self.padding_left = padding_left;
    }
}

impl AxisLabels<f64> {
    pub fn render(&self) -> Result<String, String> {
        return float_axis_labels(
            self.x_min,
            self.x_max,
            self.available_space,
            self.padding_left,
            self.vertical_direction,
            &self.unit,
        );
    }
}

impl AxisLabels<DateTime<Local>> {
    pub fn render(&self) -> Result<String, String> {
        return datetime_axis_labels(
            self.x_min,
            self.x_max,
            self.available_space,
            self.padding_left,
            self.vertical_direction,
            &self.unit,
        );
    }
}
