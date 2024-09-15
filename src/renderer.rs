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
        let label_len = label_strs[i].len() as i32;
        let middle_index = ((available_space as f64) * (label - x_min) / (x_max - x_min)) as i32;
        let offset = middle_index - label_len;
        if offset < 0 || (offset + label_len >= available_space) {
            found_overlap = true;
            // Does not fit, skip drawing this number
            continue;
        }
        // Write label string to result
        let range_for_writing = (offset as usize)..((offset + label_len) as usize);
        if result[range_for_writing.clone()].trim().is_empty() {
            result.replace_range(range_for_writing, &label_strs[i]);
        } else {
            found_overlap = true;
        }
        // TODO Set `found_overlap` to true already when there is zero spacing between labels.
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
}

fn format_float(x: f64, nr_digits: usize) -> String {
    return format!("{0:.1$}", x, nr_digits);
}

fn vec_unique(vector: &Vec<String>) -> bool {
    if vector.len() == 0 {
        return true;
    }
    let mut copy = vector.clone();
    copy.dedup();
    return vector.len() == copy.len();
}
