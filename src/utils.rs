use rand::rngs::StdRng;
use rand::Rng;

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

    pub fn choose(vec: &Vec<f64>, rng: &mut StdRng) -> usize {
        debug_assert!(vec.iter().sum::<f64>() - 1.0 < 1e-6,
            "Probabilities do not sum to 1.0: {:?}", vec);
        let mut sample = rng.gen_range(0.0..1.0);
        let mut i = 0;

        while sample > 0.0 {
            sample -= vec[i];
            i += 1;
        }

        i - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_arg_max() {
        let vec = vec![0.0, 1.0, 20.0, 3.0, -4.0];
        assert_eq!(Utils::arg_max(&vec), 2);
    }

    #[test]
    fn test_choose_normal_case() {
        let weights = vec![0.1, 0.2, 0.3, 0.4];
        let mut rng = StdRng::seed_from_u64(42); // Fixed seed for reproducibility

        // Test multiple times to ensure the distribution is correct
        let mut counts = vec![0; weights.len()];
        for _ in 0..10000 {
            let index = Utils::choose(&weights, &mut rng);
            counts[index] += 1;
        }

        // Check that the distribution is roughly proportional to the weights
        for (i, &count) in counts.iter().enumerate() {
            let expected = (weights[i] * 10000.0).round() as usize;
            assert!((count as i32 - expected as i32).abs() < 500, "Index {} was chosen {} times, expected ~{}", i, count, expected);
        }
    }


    #[test]
    fn test_choose_single_element() {
        let weights = vec![1.0];
        let mut rng = StdRng::seed_from_u64(42);

        // The only possible result is index 0
        assert_eq!(Utils::choose(&weights, &mut rng), 0);
    }

    #[test]
    fn test_choose_zero_weights() {
        let weights = vec![0.0, 0.0, 1.0, 0.0];
        let mut rng = StdRng::seed_from_u64(42);

        // The only possible result is index 2
        assert_eq!(Utils::choose(&weights, &mut rng), 2);
    }
}