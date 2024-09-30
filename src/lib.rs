mod rendering;
mod scoring;
mod utils;

const MAX_SKIP_AMOUNT: i32 = 9;
const Q_VALUES: [f64; 6] = [1.0, 5.0, 2.0, 2.5, 4.0, 3.0];

pub fn float_axis_labels(x_min: f64, x_max: f64, available_space: i32) -> String {
    let base_exponent = (x_max - x_min).log10() as i64;
    // println!("DEBUG: base_exponent = {base_exponent}");
    let preferred_nr_labels =
        crate::scoring::compute_preferred_number_of_labels(available_space, false);
    // println!("DEBUG: preferred_nr_labels = {preferred_nr_labels}");

    let mut best_score = -2.0;
    let mut best_labels: Vec<f64> = vec![];
    let mut best_result = String::new();
    let mut q: f64;
    let mut step_size: f64;
    for exponent in [base_exponent, base_exponent - 1] {
        // Find closest "zero" and thus the start of the label generation
        let f = x_min / 10_f64.powf(exponent as f64 + 1.0);
        let label_start = f.floor() * 10_f64.powf(exponent as f64 + 1.0);
        // println!("DEBUG: exponent = {exponent}, f = {f}, label_start = {label_start}");

        // j is the "skip amount"
        for j in 1..(MAX_SKIP_AMOUNT + 1) {
            // i is the index of the currently selected "nice" number q for i, q in
            // enumerate(Q_VALUES):
            for ix in 0..(Q_VALUES.len()) {
                let i = ix as i32;
                q = Q_VALUES[ix];
                step_size = q * (j as f64) * 10_f64.powf(exponent as f64);
                let labels = crate::utils::linspace(label_start, x_min, x_max, step_size);
                if labels.len() < 2 {
                    // A single label is not meaningful
                    continue;
                }
                // println!("\nDEBUG: Checking labels {:?} ...", labels);

                let simplicity =
                    crate::scoring::compute_simplicity_score(&labels, i, j, Q_VALUES.len());
                assert!(simplicity <= 1.0);
                let coverage = crate::scoring::compute_coverage_score(&labels, x_min, x_max);
                assert!(coverage <= 1.0);
                let density = crate::scoring::compute_density_score(&labels, preferred_nr_labels);
                assert!(density <= 1.0);
                // println!(
                //     "-> simplicity = {simplicity}, coverage = {coverage}, density = {density}"
                //  );
                let score_upper_bound =
                    crate::scoring::upper_bound_on_overall_score(simplicity, coverage, density);
                assert!(score_upper_bound <= 1.0);
                // println!("-> score_upper_bound = {score_upper_bound}");
                if (best_labels.len() > 0) && (score_upper_bound < best_score) {
                    continue;
                }

                // We may have found a new best label set, depending on the last score, which is
                // `grid_alignment`.
                let (result, grid_overlap) =
                    crate::rendering::render(&labels, x_min, x_max, available_space);
                // TODO Full alignment score incliding regularity
                let grid_alignment = 1.0 - ((grid_overlap as i32) as f64);
                assert!(grid_alignment <= 1.0);
                let score =
                    crate::scoring::overall_score(simplicity, coverage, density, grid_alignment);
                assert!(score <= 1.0);
                // println!("-> score = {score}");
                if score > best_score {
                    best_labels = labels;
                    best_score = score;
                    best_result = result;
                    // println!("Found best label set! 😀 New favorite: {:?}", best_result);
                }
            }
        }
    }
    // println!("-> Best labels: {:?}", best_labels);
    // println!("-> Rendered as: {best_result}");

    return best_result;
}
