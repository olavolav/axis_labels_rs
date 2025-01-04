use axis_labels_rs::AxisLabels;

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

    for _ in 0..nr_runs {
        max *= 1.05;
        let labels = AxisLabels::new(min, max, space, false).with_unit(String::from(" m"));
        println!("{}", labels.render().unwrap());
    }
}
