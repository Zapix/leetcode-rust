struct Solution;

impl Solution {
    fn count_valid_pairs(threshold: i32, nums: &Vec<i32>) -> i32 {
        let mut count = 0;
        let mut index = 0;
        while index < nums.len() - 1 {
            if nums[index + 1] - nums[index] <= threshold {
                count += 1;
                index += 1;
            }
            index += 1;
        }
        count
    }

    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        nums.sort();
        
        let mut left = 0;
        let mut right = nums.last().unwrap() - nums.first().unwrap();
        while left < right {
            let mid = (left + right) / 2;
            if Self::count_valid_pairs(mid, &nums) >= p {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::minimize_max(vec![10, 1, 2, 7, 1, 3], 2), 1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::minimize_max(vec![4, 2, 1, 2], 1), 0);
    }
    
    #[test]
    fn test_example_3() {
        assert_eq!(Solution::minimize_max(vec![4,0,2,1,2,5,5,3], 3), 1);
    }
}
