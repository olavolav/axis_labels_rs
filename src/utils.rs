pub fn linspace(zero_point: f64, min: f64, max: f64, step: f64) -> Vec<f64> {
    let mut vec: Vec<f64> = Vec::new();
    let mut x = zero_point;

    while x < max {
        if x >= min {
            // This accumulates numerical error, but should be fine for now.
            vec.push(x);
        }
        x += step;
    }

    return vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mismatching_limits_should_yield_empty_vector() {
        let space = linspace(0.0, 1.0, -3.0, 0.55);
        assert_eq!(space.len(), 0);
    }

    #[test]
    fn simple_linspace() {
        let space = linspace(0.0, 0.9, 2.1, 1.0);
        assert_eq!(space.len(), 2);
        assert!((space[0] - 1.0).abs() < 1e6);
        assert!((space[1] - 2.0).abs() < 1e6);
    }

    #[test]
    fn simple_linspace_with_a_bit_of_offset() {
        let space = linspace(0.5, 0.9, 2.9, 1.0);
        assert_eq!(space.len(), 2);
        assert!((space[0] - 1.5).abs() < 1e6);
        assert!((space[1] - 2.5).abs() < 1e6);
    }

    #[test]
    fn linspace_should_be_numberically_stable() {
        let space = linspace(0.0, 9_000.0, 10_000.1, 1.0);
        assert!((space[space.len() - 1] - 10_000.0).abs() < 1e6);
    }
}
