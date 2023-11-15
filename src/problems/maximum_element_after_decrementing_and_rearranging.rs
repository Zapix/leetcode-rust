use std::collections::HashSet;
#[allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let mut arr = arr.clone();
        arr.sort();
        let mut ans: i32 = 1;
        for i in 1..arr.len() {
            if arr[i] >= ans + 1 {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::maximum_element_after_decrementing_and_rearranging::Solution;

    #[test]
    fn simple_1() {
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![2,2,1,2,1]),
            2
        );
    }

    #[test]
    fn simple_2() {
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![1,100,1000]),
            3
        );
    }

    #[test]
    fn simple_3() {
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![1,2,3,4,5]),
            5
        );
    }

    #[test]
    fn simple_4()  {
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![10, 10, 10, 10, 10, 10]),
            6
        );
    }
}