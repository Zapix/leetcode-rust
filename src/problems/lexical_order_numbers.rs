#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = vec![];
        for i in 1..10 {
            if i <= n {
                result.push(i);
                Solution::fill_next_numbers(n, i * 10, &mut result);
            }
        }
        result
    }

    fn fill_next_numbers(n: i32, base: i32, result: &mut Vec<i32>) {
        for i in 0..10 {
            if base + i <= n {
                result.push(base + i);
                Solution::fill_next_numbers(n, (base + i) * 10, result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::lexical_order(13),
            vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(Solution::lexical_order(2), vec![1, 2]);
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::lexical_order(20),
            vec![1, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 2, 20, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn leetcode_4() {
        assert_eq!(
            Solution::lexical_order(100),
            vec![
                1, 10, 100, 11, 12, 13, 14, 15, 16, 17, 18, 19, 2, 20, 21, 22, 23, 24, 25, 26, 27,
                28, 29, 3, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 4, 40, 41, 42, 43, 44, 45, 46,
                47, 48, 49, 5, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 6, 60, 61, 62, 63, 64, 65,
                66, 67, 68, 69, 7, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 8, 80, 81, 82, 83, 84,
                85, 86, 87, 88, 89, 9, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99
            ]
        );
    }
}
