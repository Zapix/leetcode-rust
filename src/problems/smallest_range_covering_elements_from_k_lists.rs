use std::collections::HashMap;

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let k = nums.len();
        let mut merged = nums
            .iter()
            .enumerate()
            .map(|(i, x)| x.iter().map(move |val| (*val, i)))
            .fold(Vec::new(), |mut acc, x| {
                acc.extend(x);
                acc
            });
        merged.sort();
        let mut left = 0;
        let mut right = 0;
        let mut window = HashMap::new();
        let mut min_range: Option<(i32, i32)> = None;
        while right < merged.len() {
            if window.len() < k {
                let (_, num) = merged[right];
                if window.contains_key(&num) {
                    let k = window.get_mut(&num).unwrap();
                    *k += 1;
                } else {
                    window.insert(num, 1);
                }

                while window.len() == k {
                    let (left_value, num) = merged[left];
                    let (right_value, _) = merged[right];
                    min_range = match min_range {
                        None => Some((left_value, right_value)),
                        Some((left_current, right_current)) => {
                            if (left_current - right_current).abs()
                                > (left_value - right_value).abs()
                            {
                                Some((left_value, right_value))
                            } else {
                                Some((left_current, right_current))
                            }
                        }
                    };
                    let k = window.get_mut(&num).unwrap();
                    *k -= 1;
                    if *k == 0 {
                        window.remove(&num);
                    }
                    left += 1;
                }

                right += 1;
            }
        }
        min_range.map(|x| vec![x.0, x.1]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_range() {
        let nums = vec![
            vec![4, 10, 15, 24, 26],
            vec![0, 9, 12, 20],
            vec![5, 18, 22, 30],
        ];
        let expected = vec![20, 24];
        assert_eq!(Solution::smallest_range(nums), expected);
    }

    #[test]
    fn test_smallest_range_all_same() {
        let nums = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
        let expected = vec![1, 1];
        assert_eq!(Solution::smallest_range(nums), expected);
    }
}
