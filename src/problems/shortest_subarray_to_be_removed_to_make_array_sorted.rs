#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let mut prev = &0;
        let mut left_increase = 0;
        while left_increase < arr.len() && arr[left_increase] >= *prev {
            prev = &arr[left_increase];
            left_increase += 1;
        }
        if left_increase == arr.len() {
            return 0;
        }

        prev = &1000000001;
        let mut right_decrease = arr.len() - 1;
        while right_decrease >= 0 && arr[right_decrease] <= *prev {
            prev = &arr[right_decrease];
            right_decrease -= 1;
        }
        right_decrease += 1;

        let mut result = (arr.len() - 1) as i32;

        for (i, num) in vec![0]
            .iter()
            .chain(arr[0..(left_increase)].iter())
            .enumerate()
        {
            let partition = arr[right_decrease..].partition_point(|x| *x < *num) + right_decrease;
            result = result.min(partition as i32 - i as i32)
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let arr = vec![1, 2, 3, 10, 4, 2, 3, 5];
        assert_eq!(Solution::find_length_of_shortest_subarray(arr), 3);
    }

    #[test]
    fn test_example_2() {
        let arr = vec![5, 4, 3, 2, 1];
        assert_eq!(Solution::find_length_of_shortest_subarray(arr), 4);
    }

    #[test]
    fn test_example_3() {
        let arr = vec![1, 2, 3];
        assert_eq!(Solution::find_length_of_shortest_subarray(arr), 0);
    }

    #[test]
    fn test_example_4() {
        let arr = vec![1, 2, 3, 99, 4, 5, 6, 99, 7, 8, 9, 10];
        assert_eq!(Solution::find_length_of_shortest_subarray(arr), 5);
    }
}
