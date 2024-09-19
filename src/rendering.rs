pub fn render(labels: &Vec<f64>, x_min: f64, x_max: f64, available_space: i32) -> (String, bool) {
    // Initialize the empty string
    let mut result = String::new();
    for _ in 0..available_space {
        result.push_str(" ");
    }
    let mut found_overlap = false;

    // Find string labels
    let label_strs = find_shortest_string_representation(labels);

    // render the individual numbers
    for i in 0..labels.len() {
        let label = labels[i];
        let mut label_len = label_strs[i].len() as i32;
        let middle_index = ((available_space as f64) * (label - x_min) / (x_max - x_min)) as i32;
        let mut offset = middle_index - label_len;
        if offset < 0 || (offset + label_len >= available_space) {
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

fn find_shortest_string_representation(labels: &Vec<f64>) -> Vec<String> {
    // NOTE This assumes the labels to be equidistant
    let mut str_labels: Vec<String> = Vec::new();
    for _ in 0..labels.len() {
        str_labels.push(String::from(""));
    }
    for nr_digits in 0..10 {
        for i in 0..labels.len() {
            str_labels[i] = format_float(labels[i], nr_digits);
        }
        if vec_unique(&str_labels) {
            return str_labels;
        }
    }
    return str_labels;
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

fn vec_unique(vector: &Vec<String>) -> bool {
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
    fn shortest_string_representation_of_integers() {
        let labels = vec![1.0, 2.0, 3.0];
        let ls = find_shortest_string_representation(&labels);
        assert_eq!(ls.len(), 3);
        assert_eq!(ls[0], "1");
        assert_eq!(ls[1], "2");
        assert_eq!(ls[2], "3");
    }

    #[test]
    fn shortest_string_representation_of_half_integers() {
        let labels = vec![1.0, 1.5, 2.0];
        let ls = find_shortest_string_representation(&labels);
        assert_eq!(ls.len(), 3);
        assert_eq!(ls[0], "1.0");
        assert_eq!(ls[1], "1.5");
        assert_eq!(ls[2], "2.0");
    }

    #[test]
    fn shortest_string_representation_of_fractions() {
        let labels = vec![1.0, 2.0, 3.0001];
        let ls = find_shortest_string_representation(&labels);
        assert_eq!(ls.len(), 3);
        assert_eq!(ls[0], "1");
        assert_eq!(ls[1], "2");
        assert_eq!(ls[2], "3");
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
