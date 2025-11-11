use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn count_zeros_ones(s: &str) -> (i32, i32) {
        let mut zeros = 0;
        let mut ones = 0;
        for c in s.chars() {
            if c == '0' {
                zeros += 1;
            } else if c == '1' {
                ones += 1;
            }
        }
        (zeros, ones)
    }

    pub fn dp(
        values: &Vec<(i32, i32)>,
        m: i32,
        n: i32,
        index: usize,
        hash_map_dp: &mut HashMap<(i32, i32, usize), i32>,
    ) -> i32 {
        if index >= values.len() {
            return 0;
        }
        if let Some(&result) = hash_map_dp.get(&(m, n, index)) {
            return result;
        }
        let (zeros, ones) = values[index];
        let mut include_current = 0;
        if m >= zeros && n >= ones {
            include_current = 1 + Self::dp(values, m - zeros, n - ones, index + 1, hash_map_dp);
        }
        let exclude_current = Self::dp(values, m, n, index + 1, hash_map_dp);
        let result = include_current.max(exclude_current);
        hash_map_dp.insert((m, n, index), result);
        result
    }

    pub fn find_max_from(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut values = strs
            .into_iter()
            .map(|s| Self::count_zeros_ones(s.as_str()))
            .collect::<Vec<(i32, i32)>>();

        let mut hash_map_dp = HashMap::new();

        Self::dp(&values, m, n, 0, &mut hash_map_dp)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_max_from() {
        let strs = vec![
            "10".to_string(),
            "0001".to_string(),
            "111001".to_string(),
            "1".to_string(),
            "0".to_string(),
        ];
        let m = 5;
        let n = 3;
        let result = Solution::find_max_from(strs, m, n);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_find_max() {
        let strs = vec!["10".to_string(), "0".to_string(), "1".to_string()];
        let m = 1;
        let n = 1;
        let result = Solution::find_max_from(strs, m, n);
        assert_eq!(result, 2);
    }
}
