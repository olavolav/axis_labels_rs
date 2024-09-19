/// Compute upper bound to full score of labels based on partial properties
pub fn upper_bound_on_overall_score(simplicity: f64, coverage: f64, density: f64) -> f64 {
    return overall_score(simplicity, coverage, density, 1.0);
}

/// Compute full score of labels based on properties
pub fn overall_score(simplicity: f64, coverage: f64, density: f64, alignment: f64) -> f64 {
    return simplicity * 0.4 + coverage * 0.25 + density * 0.3 + alignment * 0.2;
}

/// Compute an estimate for the preferred number of labels.
pub fn compute_preferred_number_of_labels(available_space: i32, vertical_direction: bool) -> i32 {
    let best_spacing = if vertical_direction { 5.6 } else { 15.0 };
    let preferred_nr_labels = ((available_space as f32) / best_spacing) as i32;

    return std::cmp::max(2, std::cmp::min(20, preferred_nr_labels));
}

/// Simplicity score according to Talbot.
pub fn compute_simplicity_score(_labels: &Vec<f64>, i: i32, j: i32, q_len: usize) -> f64 {
    // Indicator variable that is one if zero is part of the labels, and zero otherwise
    // NOTE It might make sense to extend this to all gridline values, plus zero
    let v = 0.0; // TODO (any(np.isclose(labels, np.zeros(len(labels)))) as usize);
    return 1.0 - ((i as f64) - 1.0) / ((q_len as f64) - 1.0) - (j as f64) + v;
}

/// Coverage score according to Talbot.
pub fn compute_coverage_score(labels: &Vec<f64>, x_min: f64, x_max: f64) -> f64 {
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
pub fn compute_density_score(labels: &Vec<f64>, preferred_nr: i32) -> f64 {
    let n = labels.len() as f64;
    let p = preferred_nr as f64;
    return 1.0 - f64::max(n / p, p / n);
}
