struct Solution;

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let chunks = nums
            .as_slice()
            .chunks(3)
            .map(|chunk| {
                if chunk[2] - chunk[0] <= k{
                    Ok(chunk.into())
                } else {
                    Err(())
                }
            })
            .collect::<Result<Vec<Vec<i32>>, ()>>();

        chunks.unwrap_or(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_array_example_1() {
        let nums = vec![1, 3, 4, 8, 7, 9, 3, 5, 1];
        let k = 2;
        let expected = vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]];
        assert_eq!(Solution::divide_array(nums, k), expected);
    }

    #[test]
    fn test_divide_array_example_2() {
        let nums = vec![2, 4, 2, 2, 5, 2];
        let k = 2;
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::divide_array(nums, k), expected);
    }
}
