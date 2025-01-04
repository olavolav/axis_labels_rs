use axis_labels_rs::AxisLabels;

/// This is the simple example from the Readme.
///
/// Run via:
///
/// ```
/// $ cargo run --example 1-from_readme
/// ```
fn main() {
    let labels = AxisLabels::new(1.0, 123.4, 60, false).render();
    println!("{}", labels.unwrap());
}
