use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        let sorted_nums = sorted_nums;

        let mut group = 0;
        let mut num_to_group: HashMap<i32, i32> = HashMap::new();
        let mut group_to_que: HashMap<i32, VecDeque<i32>> = HashMap::new();
        num_to_group.insert(sorted_nums[0], group);
        let entry = group_to_que.entry(group).or_insert(VecDeque::new());
        entry.push_back(sorted_nums[0]);
        for i in 1..sorted_nums.len() {
            if (sorted_nums[i - 1] - sorted_nums[i]).abs() > limit {
                group += 1;
            }
            num_to_group.insert(sorted_nums[i], group);
            let entry = group_to_que.entry(group).or_insert(VecDeque::new());
            entry.push_back(sorted_nums[i]);
        }

        let mut result = vec![];
        for num in nums {
            let group = num_to_group.get(&num).unwrap();
            let que = group_to_que.entry(*group).or_insert(VecDeque::new());
            result.push(que.pop_front().unwrap());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 5, 3, 9, 8];
        let limit = 2;
        assert_eq!(
            Solution::lexicographically_smallest_array(nums, limit),
            vec![1, 3, 5, 8, 9]
        );
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 7, 6, 18, 2, 1];
        let limit = 3;
        assert_eq!(
            Solution::lexicographically_smallest_array(nums, limit),
            vec![1, 6, 7, 18, 1, 2]
        );
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 7, 28, 19, 10];
        let limit = 3;
        assert_eq!(
            Solution::lexicographically_smallest_array(nums, limit),
            vec![1, 7, 28, 19, 10]
        );
    }
}
