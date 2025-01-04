# axis_labels_rs

[![crates.io](https://img.shields.io/crates/v/axis_labels_rs)](https://crates.io/crates/axis_labels_rs)


This is a library to compute optimally readable axis labels for terminal plots.
The code is based on the axis_labels code of the Python plotting library
[uniplot](https://github.com/olavolav/uniplot).

It is implemented in Rust for performance reasons, since finding optimal axis
labels is done by testing & scoring a large number of possible labels.

## Example

```rust
use axis_labels_rs::AxisLabels;

let labels = AxisLabels::new(1.0, 123.4, 60, false).render();
println!("{}", labels.unwrap());
```
yields readable axis labels:
```
          25           50          75          100
```
