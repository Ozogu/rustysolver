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