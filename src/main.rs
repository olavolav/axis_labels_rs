fn main() {
    println!("###### axis_labels_rs ######");
    let min = 0.023;
    let max = 122.1;
    println!("DEBUG: min = {min}, max = {max}");
    println!("{}", float_axis_labels(min, max, 60));
}

const MAX_SKIP_AMOUNT: i32 = 9;
const Q_VALUES: [f64; 6] = [1.0, 5.0, 2.0, 2.5, 4.0, 3.0];

pub fn float_axis_labels(x_min: f64, x_max: f64, available_space: i32) -> String {
    let base_exponent = (x_max - x_min).log10() as i64;
    println!("DEBUG: base_exponent = {base_exponent}");
    let preferred_nr_labels = compute_preferred_number_of_labels(available_space, false);
    println!("DEBUG: preferred_nr_labels = {preferred_nr_labels}");

    let mut best_score = -0.2;
    let mut best_labels: Vec<f64> = vec![];
    let mut best_result = String::new();
    let mut q: f64;
    let mut step_size: f64;
    for exponent in [base_exponent, base_exponent - 1] {
        // Find closest "zero" and thus the start of the label generation
        let f = x_min / 10_f64.powf(exponent as f64 + 1.0);
        let label_start = f.floor() * 10_f64.powf(exponent as f64 + 1.0);
        println!("DEBUG: exponent = {exponent}, f = {f}, label_start = {label_start}");

        // j is the "skip amount"
        for j in 1..(MAX_SKIP_AMOUNT + 1) {
            // i is the index of the currently selected "nice" number q for i, q in
            // enumerate(Q_VALUES):
            for ix in 0..(Q_VALUES.len()) {
                let i = ix as i32;
                q = Q_VALUES[ix];
                step_size = q * (j as f64) * 10_f64.powf(exponent as f64);
                let labels = linspace(label_start, x_min, x_max, step_size);
                if labels.len() < 2 {
                    // A single label is not meaningful
                    continue;
                }
                println!("DEBUG: Checking labels {:?} ...", labels);

                let simplicity = compute_simplicity_score(&labels, i, j);
                let coverage = compute_coverage_score(&labels, x_min, x_max);
                let density = compute_density_score(&labels, preferred_nr_labels);
                // println!(
                //     "-> simplicity = {simplicity}, coverage = {coverage}, density = {density}"
                // );
                let score_upper_bound = upper_bound_on_overall_score(simplicity, coverage, density);
                // println!("-> score_upper_bound = {score_upper_bound}");
                if (best_labels.len() > 0) && (score_upper_bound < best_score) {
                    continue;
                }

                // We may have found a new best label set, depending on the last score, which is
                // `grid_alignment`.
                let (result, grid_overlap) = render(&best_labels, x_min, x_max, available_space);
                // TODO Full alignment score incliding regularity
                let grid_alignment = 1.0 - ((grid_overlap as i32) as f64);
                let score = overall_score(simplicity, coverage, density, grid_alignment);
                if score < best_score {
                    continue;
                }

                println!("Found best label set! 😀");
                best_labels = labels.clone();
                best_score = score_upper_bound;
                best_result = result.clone();
            }
        }
    }
    println!("-> Best labels: {:?}", best_labels);
    println!("-> Rendered as: {best_result}");

    return best_result;
}

fn linspace(zero_point: f64, min: f64, max: f64, step: f64) -> Vec<f64> {
    let mut vec: Vec<f64> = Vec::new();
    let mut x = zero_point;

    while x < max {
        if x >= min {
            // This accumulates numerical error, but should be fine for now.
            vec.push(x);
        }
        x += step;
    }

    return vec;
}

/// Compute upper bound to full score of labels based on partial properties
fn upper_bound_on_overall_score(simplicity: f64, coverage: f64, density: f64) -> f64 {
    return overall_score(simplicity, coverage, density, 1.0);
}

/// Compute full score of labels based on properties
fn overall_score(simplicity: f64, coverage: f64, density: f64, alignment: f64) -> f64 {
    return simplicity * 0.4 + coverage * 0.25 + density * 0.3 + alignment * 0.2;
}

/// Compute an estimate for the preferred number of labels.
fn compute_preferred_number_of_labels(available_space: i32, vertical_direction: bool) -> i32 {
    let best_spacing = if vertical_direction { 5.6 } else { 15.0 };
    let preferred_nr_labels = ((available_space as f32) / best_spacing) as i32;

    return std::cmp::max(2, std::cmp::min(20, preferred_nr_labels));
}

/// Simplicity score according to Talbot.
fn compute_simplicity_score(_labels: &Vec<f64>, i: i32, j: i32) -> f64 {
    // Indicator variable that is one if zero is part of the labels, and zero otherwise
    // NOTE It might make sense to extend this to all gridline values, plus zero
    let v = 0.0; // TODO (any(np.isclose(labels, np.zeros(len(labels)))) as usize);
    return 1.0 - ((i as f64) - 1.0) / ((Q_VALUES.len() as f64) - 1.0) - (j as f64) + v;
}

/// Coverage score according to Talbot.
fn compute_coverage_score(labels: &Vec<f64>, x_min: f64, x_max: f64) -> f64 {
    if labels.len() < 2 {
        return 0.0;
    }
    // Here we can safely unwrap
    let l0 = labels.first().unwrap();
    let l1 = labels.last().unwrap();
    return 1.0
        - 5.0 * ((x_max - l1).powf(2.0) + (x_min - l0).powf(2.0)) / ((x_max - x_min).powf(2.0));
}

/// Density score according to Talbot.
fn compute_density_score(labels: &Vec<f64>, preferred_nr: i32) -> f64 {
    let n = labels.len() as f64;
    let p = preferred_nr as f64;
    return 1.0 - f64::max(n / p, p / n);
}

fn render(labels: &Vec<f64>, x_min: f64, x_max: f64, available_space: i32) -> (String, bool) {
    // Initialize the empty string
    let mut result = String::new();
    for _ in 0..available_space {
        result.push_str(" ");

    }
    let mut found_overlap = false;

    // render the individual numbers
    for label in labels {
        let middle_index = ((available_space as f64) * (label-x_min)/(x_max-x_min)) as i32;
        let label_str = format!("{:.2}", label);
        let label_len = label_str.len() as i32;
        let offset = middle_index - label_len;
        if offset < 0 || (offset + label_len >= available_space) {
            found_overlap = true;
            // Does not fit, skip drawing this number
            continue;
        }
        // Write label string to result
        let range_for_writing = (offset as usize)..((offset+label_len) as usize);
        if result[range_for_writing.clone()].trim().is_empty() {
            result.replace_range(range_for_writing, &label_str);
        } else {
            found_overlap = true;
        }

    }

    return (result, found_overlap);
}
