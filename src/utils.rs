pub struct Utils {}

impl Utils {
    pub fn arg_max(vec: &Vec<f64>) -> usize {
        let mut max = vec[0];
        let mut max_index = 0;

        for i in 1..vec.len() {
            if vec[i] > max {
                max = vec[i];
                max_index = i;
            }
        }

        max_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_max() {
        let vec = vec![0.0, 1.0, 20.0, 3.0, -4.0];
        assert_eq!(Utils::arg_max(&vec), 2);
    }
}