use std::collections::HashMap;

trait ToDigits {
    fn to_digits(&self) -> Vec<i32>;
}

impl ToDigits for i32 {
    fn to_digits(&self) -> Vec<i32> {
        let mut num = *self;
        let mut digits = Vec::new();
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = -1;
        let mut value_map: HashMap<i32, Option<i32>> = HashMap::new();
        for num in nums {
            let sum = num.to_digits().iter().sum();
            let entry = value_map.entry(sum).or_insert(None);
            match entry {
                None => {
                    *entry = Some(num);
                }
                Some(value) => {
                    max_sum = max_sum.max(*value + num);
                    *entry = Some((*value).max(num));
                }
            }
        }
        max_sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_sum() {
        assert_eq!(Solution::maximum_sum(vec![18, 43, 36, 13, 7]), 54);
        assert_eq!(Solution::maximum_sum(vec![10, 12, 19, 14]), -1);
    }
}
