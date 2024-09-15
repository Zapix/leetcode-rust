#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut max_count = 0;
        let mut i = 0usize;
        while i < nums.len() {
            if nums[i] >= max {
                if nums[i] > max {
                    max_count = 0;
                }
                max = nums[i];
                let mut j = 0usize;
                while i + j < nums.len() && nums[i + j] == max {
                    j += 1;
                }
                max_count = max_count.max(j);
                i += j;
            } else {
                i += 1;
            }
        }
        println!("max = {} count = {}", max, max_count);
        max_count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn leetcode_1() {
        let arr = vec![1, 2, 3, 3, 2, 2];
        assert_eq!(Solution::longest_subarray(arr), 2);
    }

    #[test]
    fn leetcode_2() {
        let arr = vec![1, 2, 3, 4];
        assert_eq!(Solution::longest_subarray(arr), 1);
    }

    #[test]
    fn leetcode_3() {
        let arr = vec![
            96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 279979,
        ];
        assert_eq!(Solution::longest_subarray(arr), 1);
    }
}
