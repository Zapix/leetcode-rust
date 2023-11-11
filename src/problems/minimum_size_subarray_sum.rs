#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut result: Option<i32> = None;

        let mut current = 0;
        let mut j = 0 as usize;

        for i in 0..nums.len() {
            current += nums[i];
            while j <= i && target <= current {
                let size = (i - j + 1) as i32;
                result = match result {
                    None => Some(size),
                    Some(x) => Some(x.min(size))
                };
                current -= nums[j];
                j += 1;
            }
        }

        match result {
            None => 0,
            Some(x) => x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_1() {
        assert_eq!(
            Solution::min_sub_array_len(7, vec![2,3,1,2,4,3]),
            2
        );
    }

    #[test]
    fn simple_2() {
        assert_eq!(
            Solution::min_sub_array_len(7, vec![4,3,2,1,3,2]),
            2
        );
    }

    #[test]
    fn simple_3() {
        assert_eq!(
            Solution::min_sub_array_len(1, vec![1,4,4]),
            1
        );
    }

    #[test]
    fn simple_4() {
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1,1,1,1,1,1,1,1]),
            0
        );
    }
}