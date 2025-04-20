use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        answers
            .iter()
            .fold(HashMap::new(), |mut acc, item| {
                let entry = acc.entry(*item).or_insert(0);
                *entry += 1;
                acc
            })
            .iter()
            .map(|x| {
                let rabbit_count = *x.0 + 1;
                ((x.1 / rabbit_count) + if x.1 % rabbit_count == 0 { 0 } else { 1 }) * rabbit_count
            })
            .sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let answers = vec![1, 1, 2];
        let result = Solution::num_rabbits(answers);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_example_2() {
        let answers = vec![10, 10, 10];
        let result = Solution::num_rabbits(answers);
        assert_eq!(result, 11);
    }
}
