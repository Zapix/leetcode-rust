struct Solution;

impl Solution {
    pub fn max_integer(num: i32) -> i32 {
        let digits = num.to_string().chars().collect::<Vec<_>>();
        let replace_digit = *digits.iter().find(|&x| *x != '9').unwrap_or(&'9');
        digits
            .iter()
            .map(|&x| if x == replace_digit { '9' } else { x })
            .fold(0, |acc, x| acc * 10 + (x as u8 - '0' as u8) as i32)
    }
    
    pub fn min_integer(num: i32) -> i32 {
        let digits = num.to_string().chars().collect::<Vec<_>>();
        let (&replace_digit, digit) = match digits.first() {
            Some('1') => (digits.iter().find(|&x| *x != '1' && *x != '0').unwrap_or(&'2'), '0'),
            Some(x) => (x, '1'),
            None => panic!("Unexpected condition"),
        };
        digits
            .iter()
            .map(|&x| if x == replace_digit { digit } else { x })
            .fold(0, |acc, x| acc * 10 + (x as u8 - '0' as u8) as i32)
    }
    
    pub fn max_diff(num: i32) -> i32 {
        Solution::max_integer(num) - Solution::min_integer(num)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_diff() {
        assert_eq!(Solution::max_diff(555), 888);
        assert_eq!(Solution::max_diff(9), 8);
        assert_eq!(Solution::max_diff(1), 8);
        assert_eq!(Solution::max_diff(12), 82);
        assert_eq!(Solution::max_diff(1101057), 8808050);
    }
    
    
}
