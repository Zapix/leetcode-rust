#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let greater_cnt = nums
            .iter()
            .fold(0, |acc, x| if *x <= pivot { acc } else { acc + 1 });
        let mut result = vec![pivot; nums.len()];
        let mut i = 0;
        let mut j = nums.len() - greater_cnt;
        for num in nums {
            if num < pivot {
                result[i] = num;
                i += 1;
            }
            if num > pivot {
                result[j] = num;
                j += 1;
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
        let nums = vec![9, 12, 5, 10, 14, 3, 10];
        let pivot = 10;
        assert_eq!(
            Solution::pivot_array(nums, pivot),
            vec![9, 5, 3, 10, 10, 12, 14]
        );
    }

    #[test]
    fn test_example_2() {
        let nums = vec![-3, 4, 3, 2];
        let pivot = 2;
        assert_eq!(Solution::pivot_array(nums, pivot), vec![-3, 2, 4, 3]);
    }
}
