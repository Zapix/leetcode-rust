struct Solution;

impl Solution {
    fn revamp_maximum_number(num: i32) -> i32 {
        let digits = num.to_string().chars().collect::<Vec<char>>();
        let revamp_digit = digits.iter().find(|&&x| x != '9').unwrap_or(&'9');
        digits
            .iter()
            .map(|x| if x == revamp_digit { &'9' } else { x })
            .fold(0i32, |acc, x| acc * 10 + (*x as u8 - '0' as u8) as i32)
    }
    
    fn revamp_minimum_number(num: i32) -> i32 {
        let digits = num.to_string().chars().collect::<Vec<char>>();
        let revamp_digit = digits.iter().find(|&&x| x != '0').unwrap_or(&'0');
        digits
            .iter()
            .map(|x| if x == revamp_digit { &'0' } else { x })
            .fold(0i32, |acc, x| acc * 10 + (*x as u8 - '0' as u8) as i32)
    }
    
    pub fn min_max_difference(num: i32) -> i32 {
        Solution::revamp_maximum_number(num) - Solution::revamp_minimum_number(num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let num = 11891;
        let expected = 99009;
        assert_eq!(Solution::min_max_difference(num), expected);
    }

    #[test]
    fn test_example_2() {
        let num = 90;
        let expected = 99;
        assert_eq!(Solution::min_max_difference(num), expected);
    }
    
    #[test]
    fn test_example_3() {
        let num: i32 = 99;
        let expected = 99;
        assert_eq!(Solution::min_max_difference(num), expected);
    }
}
