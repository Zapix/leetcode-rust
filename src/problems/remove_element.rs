#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut remove_cnt = 0;
        let mut j = 0 as usize;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[j] = nums[i];
                j += 1;
            } else {
                remove_cnt += 1;
            }
        }

        (nums.len() - remove_cnt) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_1() {
        let nums: &mut Vec<i32> = &mut vec![3, 2, 2, 3];
        let cnt = Solution::remove_element(nums, 3);

        assert_eq!(cnt, 2);
        assert_eq!(
            nums[0..2],
            vec![2,2]
        );
    }

    #[test]
    fn simple_2() {
        let nums: &mut Vec<i32> = &mut vec![0,1,2,2,3,0,4,2];
        let cnt = Solution::remove_element(nums, 2);

        assert_eq!(cnt, 5);
        assert_eq!(
            nums[0..5],
            vec![0,1,3,0,4]
        );
    }
}
