# axis_labels_rs

This is a library to compute optimally readable axis labels for terminal plots.
The code is based on the axis_labels code of the Python plotting library
[uniplot](https://github.com/olavolav/uniplot).

It is implemented in Rust for performance reasons, since finding optimal axis
labels is done by testing & scoring a large number of possible labels.
