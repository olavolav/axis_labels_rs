mod rendering;
mod scoring;
mod utils;

const MAX_SKIP_AMOUNT: i32 = 9;
const Q_VALUES: [f64; 6] = [1.0, 5.0, 2.0, 2.5, 4.0, 3.0];

pub fn float_axis_labels(
    x_min: f64,
    x_max: f64,
    available_space: u32,
    padding_left: u32,
    unit: &String,
) -> String {
    if (available_space == 0) || (x_max < x_min) {
        // TODO Return an actual `Result`.
        return String::from("");
    }
    let base_exponent = (x_max - x_min).log10() as i64;
    let preferred_nr_labels =
        crate::scoring::compute_preferred_number_of_labels(available_space, false);

    let mut best_score = -2.0;
    let mut best_result = String::new();
    let mut q: f64;
    let mut step_size: f64;
    for exponent in [base_exponent, base_exponent - 1] {
        // Find closest "zero" and thus the start of the label generation
        let f = x_min / 10_f64.powf(exponent as f64 + 1.0);
        let label_start = f.floor() * 10_f64.powf(exponent as f64 + 1.0);

        // j is the "skip amount"
        for j in 1..(MAX_SKIP_AMOUNT + 1) {
            // i is the index of the currently selected "nice" number q
            for ix in 0..(Q_VALUES.len()) {
                let i = ix as i32;
                q = Q_VALUES[ix];
                step_size = q * (j as f64) * 10_f64.powf(exponent as f64);
                let labels = crate::utils::linspace(label_start, x_min, x_max, step_size);
                if labels.len() < 2 {
                    // A single label is not meaningful
                    continue;
                }

                let simplicity =
                    crate::scoring::compute_simplicity_score(&labels, i, j, Q_VALUES.len());
                let coverage = crate::scoring::compute_coverage_score(&labels, x_min, x_max);
                let density = crate::scoring::compute_density_score(&labels, preferred_nr_labels);
                let score_upper_bound =
                    crate::scoring::upper_bound_on_overall_score(simplicity, coverage, density);
                if (!best_result.is_empty()) && (score_upper_bound < best_score) {
                    continue;
                }

                // We may have found a new best label set, depending on the last score, which is
                // `grid_alignment`.
                let (result, grid_overlap) = crate::rendering::render(
                    &labels,
                    x_min,
                    x_max,
                    available_space,
                    padding_left,
                    &unit,
                );
                // TODO Full alignment score incliding regularity
                let grid_alignment = 1.0 - ((grid_overlap as i32) as f64);
                let score =
                    crate::scoring::overall_score(simplicity, coverage, density, grid_alignment);
                if score > best_score {
                    best_score = score;
                    best_result = result;
                }
            }
        }
    }
    return best_result;
}
