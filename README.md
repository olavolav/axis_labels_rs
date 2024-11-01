# axis_labels_rs

[![crates.io](https://img.shields.io/crates/v/axis_labels_rs)](https://crates.io/crates/axis_labels_rs)


This is a library to compute optimally readable axis labels for terminal plots.
The code is based on the axis_labels code of the Python plotting library
[uniplot](https://github.com/olavolav/uniplot).

It is implemented in Rust for performance reasons, since finding optimal axis
labels is done by testing & scoring a large number of possible labels.

## Example

```rust
use axis_labels_rs::float_axis_labels;

let labels = float_axis_labels(0.0, 123.4, 60, 1, false, &String::from(" m"));
println!("{}", labels);
```
yields
```
0 m                    50 m                    100 m
```
