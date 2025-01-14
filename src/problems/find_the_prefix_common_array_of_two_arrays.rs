#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut a_nums = 0u64;
        let mut b_nums = 0u64;
        let mut result = vec![];
        for (a, b) in a.iter().zip(b.iter()) {
            a_nums |= 1 << (*a as u64);
            b_nums |= 1 << (*b as u64);
            result.push((a_nums & b_nums).count_ones() as i32)
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let a = vec![1, 3, 2, 4];
        let b = vec![3, 1, 2, 4];
        assert_eq!(
            Solution::find_the_prefix_common_array(a, b),
            vec![0, 2, 3, 4]
        );
    }

    #[test]
    fn test_example_2() {
        let a = vec![2, 3, 1];
        let b = vec![3, 1, 2];
        assert_eq!(Solution::find_the_prefix_common_array(a, b), vec![0, 1, 3]);
    }
}
