struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut prev_time = 0;
        let mut prev_color = ' ';
        for (c, &t) in colors.chars().zip(needed_time.iter()) {
            if c == prev_color {
                result += t.min(prev_time);
                prev_time = prev_time.max(t);
            } else {
                prev_color = c;
                prev_time = t;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5]),
            3
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::min_cost("abc".to_string(), vec![1, 2, 3]),
            0
        );
    }
}
