#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        let mut result = vec![];
        for (i, c) in input.chars().enumerate() {
            if c == '+' || c == '-' || c == '*' {
                let left = Solution::diff_ways_to_compute(input[..i].to_string());
                let right = Solution::diff_ways_to_compute(input[i + 1..].to_string());
                for l in left.iter() {
                    for r in right.iter() {
                        match c {
                            '+' => result.push(l + r),
                            '-' => result.push(l - r),
                            '*' => result.push(l * r),
                            _ => {}
                        }
                    }
                }
            }
        }
        if result.is_empty() {
            result.push(input.parse().unwrap());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        assert_eq!(
            Solution::diff_ways_to_compute("2-1-1".to_string()),
            vec![2, 0]
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::diff_ways_to_compute("2*3-4*5".to_string()),
            vec![-34, -10, -14, -10, 10]
        );
    }
}
