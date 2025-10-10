struct Solution;

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let mut values = vec![None; k as usize];
        for (i, &e) in energy.iter().enumerate() {
            let idx = (i as i32 % k) as usize;
            values[idx] = match values[idx] {
                Some(v) => Some(e.max(v + e)),
                None => Some(e),
            };
        }
        values.into_iter().filter_map(|x| x).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_energy() {
        let energy = vec![1, 2, 3, 4, 5];
        let k = 2;
        assert_eq!(Solution::maximum_energy(energy, k), 9);
    }

    #[test]
    fn test_maximum_energy_1() {
        let energy = vec![5, 2, -10, -5, 1];
        let k = 3;
        assert_eq!(Solution::maximum_energy(energy, k), 3);
    }

    fn test_maximum_energy_2() {
        let energy = vec![-2, -3, -1];
        let k = 2;
        assert_eq!(Solution::maximum_energy(energy, k), -1);
    }
}
