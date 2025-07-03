use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn find_lhs(mut nums: Vec<i32>) -> i32 {
        let counter = nums.iter().fold(HashMap::new(), |mut acc, &val| {
            let entry = acc.entry(val).or_insert(0);
            *entry += 1;
            acc
        });
        counter
            .keys()
            .filter_map(|&k| {
                if counter.contains_key(&(k + 1)) {
                    Some(counter.get(&k).unwrap() + counter.get(&(k + 1)).unwrap())
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
        assert_eq!(Solution::find_lhs(nums), 5);
    }

    #[test]
    fn example2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::find_lhs(nums), 2);
    }

    #[test]
    fn example3() {
        let nums = vec![1, 1, 1, 1];
        assert_eq!(Solution::find_lhs(nums), 0);
    }
}
