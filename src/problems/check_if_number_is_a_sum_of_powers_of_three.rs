#[allow(dead_code)]
struct Solution;

const POWER_OF_THREE: [i32; 17] = [
    1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441, 1594323, 4782969, 14348907, 43046721
];

#[allow(dead_code)]
impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        for i in 0..2usize.pow(16) {
            let mut sum = 0;
            for j in 0..16 {
                if i & (1 << j) != 0 {
                    sum += POWER_OF_THREE[j];
                }
            }
            if sum == n {
                return true;
            }
        }
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_powers_of_three() {
        assert_eq!(Solution::check_powers_of_three(12), true);
        assert_eq!(Solution::check_powers_of_three(91), true);
        assert_eq!(Solution::check_powers_of_three(21), false);
        assert_eq!(Solution::check_powers_of_three(1), true);
        assert_eq!(Solution::check_powers_of_three(2), false);
        assert_eq!(Solution::check_powers_of_three(3), true);
        assert_eq!(Solution::check_powers_of_three(27), true);
        assert_eq!(Solution::check_powers_of_three(28), true);
    }
}
