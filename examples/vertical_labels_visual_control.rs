use axis_labels_rs::float_axis_labels;

/// Run a benchmark of horizontal labels
///
/// Run via:
///
/// ```
/// $ cargo run --example vertical_labels_visual_control
/// ```
fn main() {
    let min = 6.5;
    let mut max = 7.5;
    let nr_runs = 30;
    let space = 17;
    let unit = String::from("");

    for _ in 0..nr_runs {
        max *= 1.05;
        let labels = float_axis_labels(min, max, space, 1, true, &unit);
        println!("┐");
        for line in labels.unwrap().split("\n") {
            println!("│ {line}");
        }
        println!("┘");
        println!(" ");
    }
}
