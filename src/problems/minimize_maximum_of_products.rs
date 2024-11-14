#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn can_distribute(amount: i32, quantities: &Vec<i32>, n: i32) -> bool {
        let mut m = 0;

        for quantity in quantities {
            m += quantity / amount;
            if quantity % amount > 0 {
                m += 1;
            }
        }

        m <= n
    }
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut result = i32::MAX;
        let mut left = 1;
        let mut right = *quantities.iter().max().unwrap();
        while left <= right {
            let middle = (right + left) / 2;
            if Self::can_distribute(middle, &quantities, n) {
                result = middle;
                right = middle - 1;
            } else {
                left = middle + 1;
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
        let n = 6;
        let quantities = vec![11, 6];
        assert_eq!(Solution::minimized_maximum(n, quantities), 3);
    }

    #[test]
    fn test_example_2() {
        let n = 7;
        let quantities = vec![15, 10, 10];
        assert_eq!(Solution::minimized_maximum(n, quantities), 5);
    }

    #[test]
    fn test_example_3() {
        let n = 1;
        let quantities = vec![100000];
        assert_eq!(Solution::minimized_maximum(n, quantities), 100000);
    }

    #[test]
    fn test_example_4() {
        let n = 100000;
        let quantities = vec![
            4, 4, 4, 2, 4, 2, 4, 1, 5, 4, 5, 4, 1, 1, 2, 2, 4, 1, 1, 4, 5, 3, 3, 4, 1, 2,
        ];
        assert_eq!(Solution::minimized_maximum(n, quantities), 1);
    }
}
