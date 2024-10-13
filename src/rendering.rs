pub fn render(
    labels: &Vec<f64>,
    x_min: f64,
    x_max: f64,
    available_space: i32,
    padding_left: i32,
) -> (String, bool) {
    // Initialize the empty string
    let mut result = String::new();
    for _ in 0..(padding_left + available_space) {
        result.push_str(" ");
    }
    let mut found_overlap = false;

    // Find string labels
    let char_width = (x_max - x_min) / (available_space as f64);
    let label_strs = find_shortest_string_representation(labels, char_width / 2.0);

    // render the individual numbers
    for i in 0..labels.len() {
        let label = labels[i];
        let mut label_len = label_strs[i].len() as i32;
        let middle_index = ((available_space as f64) * (label - x_min) / (x_max - x_min)) as i32;
        let mut offset = middle_index - label_len / 2 + padding_left;
        if offset < 0 || (offset + label_len >= padding_left + available_space) {
            found_overlap = true;
            // Does not fit, skip drawing this number
            continue;
        }
        let mut expanded_label = label_strs[i].clone();
        if offset > 0 {
            expanded_label = String::from(" ") + &expanded_label;
            offset -= 1;
            label_len += 1;
        }
        if offset + label_len < available_space {
            expanded_label = expanded_label + &String::from(" ");
            label_len += 1;
        }
        // Write label string to result
        let range_for_writing = (offset as usize)..((offset + label_len) as usize);
        if result[range_for_writing.clone()].trim().is_empty() {
            result.replace_range(range_for_writing, &expanded_label);
        } else {
            found_overlap = true;
        }
    }

    return (result, found_overlap);
}

/////////////
// private //
/////////////

/// This finds the lowest number of digits where the labels are unique. It also checks for the
/// labels being shifted too much from the nun-rounded version, and if so adds one digit to all
/// labels.
/// NOTE This assumes the labels to be equidistant.
fn find_shortest_string_representation(labels: &Vec<f64>, max_rounding_shift: f64) -> Vec<String> {
    let mut str_labels: Vec<String> = vec![String::from(""); labels.len()];
    let mut shift_was_too_large;

    for nr_digits in 0..10 {
        shift_was_too_large = false;
        for i in 0..labels.len() {
            let rounded_label = format_float(labels[i], nr_digits);
            let rounding_shift = compute_delta_to_rounded(labels[i], nr_digits);
            if rounding_shift.abs() > max_rounding_shift {
                shift_was_too_large = true;
            }
            str_labels[i] = rounded_label;
        }
        if vec_is_unique(&str_labels) {
            // If the shift due to rounding it too large, add a digit
            if shift_was_too_large {
                for i in 0..labels.len() {
                    str_labels[i] = format_float(labels[i], nr_digits + 1);
                }
            }
            return str_labels;
        }
    }
    return str_labels;
}

fn compute_delta_to_rounded(x: f64, nr_digits: usize) -> f64 {
    let order = 10.0_f64.powi(-1 * (nr_digits as i32));
    let remainder = x % order;
    if remainder >= 0.5 * order {
        return order - remainder;
    }
    return remainder;
}

fn format_float(x: f64, nr_digits: usize) -> String {
    let raw_number_str = format!("{0:.1$}", x, nr_digits);

    // Add thousands separator
    let parts = raw_number_str.split(".").collect::<Vec<&str>>();
    let integer_part = parts[0];
    let integer_part_with_separators = integer_part
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",");

    if parts.len() < 2 {
        return integer_part_with_separators;
    }
    let fractional_part = parts[1];
    return integer_part_with_separators + "." + fractional_part;
}

fn vec_is_unique(vector: &Vec<String>) -> bool {
    if vector.len() == 0 {
        return true;
    }
    let mut copy = vector.clone();
    copy.dedup();
    return vector.len() == copy.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_smoke_test() {
        let labels = vec![1.0, 2.0, 3.0];
        render(&labels, 0.1, 1.1, 40, 0);
    }

    #[test]
    fn render_test_with_padding() {
        let labels = vec![1.0, 2.0, 3.0];
        let (string, _overlap) = render(&labels, 0.1, 1.1, 40, 10);
        assert!(string[0..7].trim().is_empty());
    }

    #[test]
    fn shortest_string_representation_of_integers() {
        let labels = vec![1.0, 2.0, 3.0];
        let ls = find_shortest_string_representation(&labels, 2.0 / 60.0);
        assert_eq!(ls.len(), 3);
        assert_eq!(ls[0], "1");
        assert_eq!(ls[1], "2");
        assert_eq!(ls[2], "3");
    }

    #[test]
    fn shortest_string_representation_of_half_integers() {
        let labels = vec![1.0, 1.5, 2.0];
        let ls = find_shortest_string_representation(&labels, 1.0 / 60.0);
        assert_eq!(ls.len(), 3);
        assert_eq!(ls[0], "1.0");
        assert_eq!(ls[1], "1.5");
        assert_eq!(ls[2], "2.0");
    }

    #[test]
    fn shortest_string_representation_of_fractions() {
        let labels = vec![1.0, 2.0, 3.0001];
        let ls = find_shortest_string_representation(&labels, 2.0 / 60.0);
        assert_eq!(ls.len(), 3);
        assert_eq!(ls[0], "1");
        assert_eq!(ls[1], "2");
        assert_eq!(ls[2], "3");
    }

    #[test]
    fn compute_delta_for_integers_should_be_zero() {
        let delta = compute_delta_to_rounded(1234.0, 0);
        assert!((delta - 0.0) < 1e-6);
    }

    #[test]
    fn compute_delta_for_small_single_digit_should_be_that_digit() {
        let delta = compute_delta_to_rounded(1.111, 1);
        assert!((delta - 0.111) < 1e-6);
    }

    #[test]
    fn compute_delta_for_medium_single_digit_should_be_the_rest_to_round_up() {
        let delta = compute_delta_to_rounded(1.1155, 2);
        println!("delta = {delta}");
        assert!((delta - (1.12 - 1.1155)) < 1e-6);
    }

    #[test]
    fn compute_delta_for_high_single_digit_should_be_the_rest_to_one() {
        let delta = compute_delta_to_rounded(1.8888, 3);
        assert!((delta - (1.889 - 1.8888)) < 1e-6);
    }

    #[test]
    fn shortest_string_representation_of_fractions_when_more_than_uniqueness_is_required() {
        let labels = vec![5.0, 7.5, 10.0];
        let ls = find_shortest_string_representation(&labels, 5.0 / (2.0 * 60.0));
        assert_eq!(ls.len(), 3);
        assert_eq!(ls[0], "5.0");
        assert_eq!(ls[1], "7.5");
        assert_eq!(ls[2], "10.0");
    }

    #[test]
    fn should_render_float_to_specified_number_of_digits() {
        let x = 12.0;
        let str = format_float(x, 3);
        assert_eq!(str, "12.000");
    }

    #[test]
    fn should_render_thousand_separator() {
        let x = 1234567.0;
        let str = format_float(x, 0);
        assert_eq!(str, "1,234,567");
    }
}
