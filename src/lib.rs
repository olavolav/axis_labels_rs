mod rendering;
mod scoring;
mod utils;

const MAX_SKIP_AMOUNT: i32 = 9;
const Q_VALUES: [f64; 6] = [1.0, 5.0, 2.0, 2.5, 4.0, 3.0];


pub struct AxisLabels<T: PartialOrd>{
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
            unit: String::from("")
        }
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
        return float_axis_labels(self.x_min, self.x_max, self.available_space, self.padding_left, self.vertical_direction, &self.unit);
    }
}


/// Generate optimally readable axis labels.
pub fn float_axis_labels(
    x_min: f64,
    x_max: f64,
    available_space: u32,
    padding_left: u32,
    vertical_direction: bool,
    unit: &String,
) -> Result<String, String> {
    // Check arguments
    if available_space == 0 {
        return Err(String::from("Invalid arguments: available_space == 0."));
    }
    if x_max < x_min {
        return Err(String::from("Invalid arguments: x_max < x_min."));
    }

    let base_exponent = (x_max - x_min).log10() as i64;
    let preferred_nr_labels =
        crate::scoring::compute_preferred_number_of_labels(available_space, vertical_direction);

    let mut best_score = -2.0;
    let mut best_result = String::new();
    let mut step_size: f64;
    for exponent in [base_exponent, base_exponent - 1] {
        // Find closest "zero" and thus the start of the label generation
        let f = x_min / 10_f64.powf(exponent as f64 + 1.0);
        let label_start = f.floor() * 10_f64.powf(exponent as f64 + 1.0);

        // j is the "skip amount"
        for j in 1..(MAX_SKIP_AMOUNT + 1) {
            // ix (or later i) is the index of the currently selected "nice" number q
            for (ix, q) in Q_VALUES.iter().enumerate() {
                let i = ix as i32;
                step_size = q * (j as f64) * 10_f64.powf(exponent as f64);
                let labels = crate::utils::linspace(label_start, x_min, x_max, step_size);
                if labels.len() < 2 {
                    // A single label is not meaningful
                    continue;
                }

                let simplicity_score =
                    crate::scoring::compute_simplicity_score(&labels, i, j, Q_VALUES.len());
                let coverage_score = crate::scoring::compute_coverage_score(&labels, x_min, x_max);
                let density_score =
                    crate::scoring::compute_density_score(&labels, preferred_nr_labels);
                let score_upper_bound = crate::scoring::upper_bound_on_overall_score(
                    simplicity_score,
                    coverage_score,
                    density_score,
                );
                if (!best_result.is_empty()) && (score_upper_bound < best_score) {
                    continue;
                }

                // We may have found a new best label set, depending on the last entry in the score
                // vector, which is `grid_alignment`.
                let (result, grid_overlap) = crate::rendering::render(
                    &labels,
                    x_min,
                    x_max,
                    available_space,
                    padding_left,
                    vertical_direction,
                    &unit,
                );
                // TODO Full alignment score including regularity
                let grid_alignment_score = 1.0 - ((grid_overlap as i32) as f64);
                let score = crate::scoring::overall_score(
                    simplicity_score,
                    coverage_score,
                    density_score,
                    grid_alignment_score,
                );
                if score > best_score {
                    best_score = score;
                    best_result = result;
                }
            }
        }
    }
    return Ok(best_result);
}
