use axis_labels_rs::float_axis_labels;

/// Run a benchmark of horizontal labels
///
/// Run via:
///
/// ```
/// $ cargo run --example horizontal_labels_visual_control
/// ```
fn main() {
    let min = 6.5;
    let mut max = 7.5;
    let nr_runs = 150;
    let space = 60;
    let unit = String::from("");

    for _ in 0..nr_runs {
        max *= 1.05;
        let labels = float_axis_labels(min, max, space, 1, false, &unit);
        println!("{}", labels.unwrap());
    }
}
