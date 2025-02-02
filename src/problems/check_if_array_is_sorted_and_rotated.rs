#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let min = nums
            .iter()
            .min()
            .unwrap();
        let n = nums.len();

        nums
        .iter()
        .enumerate()
        .filter(|(_i, x)| *x == min)
        .map(|(i, _x)| i)
        .any(|i| {
            (0..(n - 1))
            .all(|j| {
                let idx = (i + j) % n;
                nums[idx] <= nums[(idx + 1) % n]
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::check(nums), true);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![2, 1, 3, 4];
        assert_eq!(Solution::check(nums), false);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::check(nums), true);
    }

    #[test]
    fn test_example_4() {
        let nums = vec![1, 1, 1];
        assert_eq!(Solution::check(nums), true);
    }

    #[test]
    fn test_example_5() {
        let nums = vec![2, 3, 4, 5, 1];
        assert_eq!(Solution::check(nums), true);
    }

    #[test]
    fn test_example_6() {
        let nums = vec![6, 10, 6];
        assert_eq!(Solution::check(nums), true);
    }
}
