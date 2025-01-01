#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut count_ones = s.chars().filter(|x| *x == '1').count() as i32;
        let mut count_zeros = 0;
        let mut max_score = 0;

        for ch in s.chars().take(s.len() - 1) {
            match ch {
                '0' => {
                    count_zeros += 1;
                },
                '1' => {
                    count_ones -= 1;
                },
                _ => {},
            }
            max_score = max_score.max(count_zeros + count_ones);
        }

        max_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_score("011101".to_string()), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_score("00111".to_string()), 5);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_score("1111".to_string()), 3);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::max_score("00".to_string()), 1);
    }
}