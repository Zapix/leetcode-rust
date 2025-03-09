#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut last_color = colors[0];
        let mut current = 1;

        for i in 1..(colors.len() + (k as usize) - 1) {
            if last_color == colors[i % colors.len()] {
                current = 1;
                continue;
            }
            current += 1;
            if current >= k {
                result += 1
            }
            last_color = colors[i % colors.len()]
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let colors = vec![0, 1, 0, 1, 0];
        let k = 3;
        assert_eq!(Solution::number_of_alternating_groups(colors, k), 3);
    }

    #[test]
    fn test_example_2() {
        let colors = vec![0, 1, 0, 0, 1, 0, 1];
        let k = 6;
        assert_eq!(Solution::number_of_alternating_groups(colors, k), 2);
    }

    #[test]
    fn test_example_3() {
        let colors = vec![1, 1, 0, 1];
        let k = 4;
        assert_eq!(Solution::number_of_alternating_groups(colors, k), 0);
    }
}
