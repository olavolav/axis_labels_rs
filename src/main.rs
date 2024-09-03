fn main() {
    println!("###### axis_labels_rs ######");
    let min = 0.0;
    let max = 1.1;
    println!("DEBUG: min = {min}, max = {max}");
    println!("{}", float_axis_labels(min, max, 60));
}

const MAX_LABELS:usize = 100;
struct FloatLabelSet {
    nr_labels: i64,
    labels: [f64;MAX_LABELS]
}

const MAX_SKIP_AMOUNT:i32 = 9;
const Q_VALUES: [f64;6] = [1.0, 5.0, 2.0, 2.5, 4.0, 3.0];

fn float_axis_labels(x_min: f64, x_max: f64, _available_space: i64) -> String {
    let data_range = x_max - x_min;
    let base_exponent = data_range.log10() as i64;
    println!("DEBUG: base_exponent = {base_exponent}");

    let mut best_score = -0.2;
    let mut best_result = FloatLabelSet{
        nr_labels: 0,
        labels: [0.0; MAX_LABELS]
    };
    let mut q: f64;
    let mut step_size: f64;
    for exponent in [base_exponent, base_exponent-1] {
        // Find closest "zero" and thus the start of the label generation
        let f = x_min / 10_f64.powf(exponent as f64 + 1.0);
        let label_start = f.floor() * 10_f64.powf(exponent as f64 + 1.0);
        println!("DEBUG: exponent = {exponent}, f = {f}, label_start = {label_start}");

        // j is the "skip amount"
        for j in 1..(MAX_SKIP_AMOUNT + 1) {
            // i is the index of the currently selected "nice" number q for i, q in
            // enumerate(Q_VALUES):
            for i in 0..(Q_VALUES.len()) {
                q = Q_VALUES[i];
                step_size = q * (j as f64) * 10_f64.powf(exponent as f64);
                let labels = linspace(label_start, x_max, step_size);
                if labels.len() < 2 {
                    // A single label is not meaningful
                    continue;
                }
                println!("DEBUG: Checking labels {:?} ...", labels);
            }
        }
    }

    return String::from("   ");
}

fn linspace(start: f64, stop: f64, step: f64) -> Vec<f64> {
    let mut vec: Vec<f64> = Vec::new();
    let mut x = start;
    while x < stop {
        vec.push(x);
        x += step;
    }

    return vec;
}
