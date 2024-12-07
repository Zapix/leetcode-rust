#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn can_split(num: &Vec<i32>, max_operations: i32, bag_size: i32) -> bool {
        let mut count_operations = 0;
        for val in num {
            if *val > bag_size {
                count_operations += (val - 1) / bag_size;
            }
        }
        max_operations >= count_operations
    }
    pub fn minimum_size(num: Vec<i32>, max_operations: i32) -> i32 {
        let mut min_num = 1;
        let mut max_num = *num.iter().max().unwrap();

        while min_num < max_num {
            let middle = (max_num + min_num) / 2;
            if Solution::can_split(&num, max_operations, middle) {
                max_num = middle
            } else {
                min_num = middle + 1
            }
        }
        max_num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![9];
        let max_operations = 2;
        assert_eq!(Solution::minimum_size(nums, max_operations), 3);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![2, 4, 8, 2];
        let max_operations = 4;
        assert_eq!(Solution::minimum_size(nums, max_operations), 2);
    }
}
