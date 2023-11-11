#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;

        while numbers[i] + numbers[j] != target {
            if numbers[i] + numbers[j] < target {
                i += 1;
            } else {
                j -= 1;
            }
        }

        vec![(i + 1) as i32, (j + 1) as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_1() {
        assert_eq!(
            Solution::two_sum(vec![2,7,11,16], 9),
            vec![1,2]
        );
    }

    #[test]
    fn simple_2() {
        assert_eq!(
            Solution::two_sum(vec![2, 3, 4], 6),
            vec![1,3]
        );
    }

    #[test]
    fn simple_3() {
        assert_eq!(
            Solution::two_sum(vec![-1,0], -1),
            vec![1,2]
        );
    }
}