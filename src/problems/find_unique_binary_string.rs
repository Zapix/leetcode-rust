#[allow(dead_code)]
struct Solution;

impl Solution {
    pub fn parse_binary(s: &String) -> i32 {
        let mut val = 0;
        for c in s.chars() {
            val <<= 1;
            val += (c as i32 - '0' as i32);
        }
        val
    }
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut val = nums.len() as i32;
        let mut nums = nums.iter().map(|x| Solution::parse_binary(x)).collect::<Vec<i32>>();
        nums.sort();

        for (i, num) in nums.iter().enumerate() {
            if i as i32 != *num {
                val = i as i32;
                break;
            }
        }
        String::from(&format!("{:#020b}", val)[(20 - nums.len())..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        assert_eq!(
            Solution::find_different_binary_string(vec![
                String::from("01"),
                String::from("10"),
            ]),
            String::from("00")
        );
    }

    #[test]
    fn sample_2() {
        assert_eq!(
            Solution::find_different_binary_string(vec![
                String::from("00"),
                String::from("01"),
            ]),
            String::from("10"),

        )
    }

    #[test]
    fn sample_3() {
        assert_eq!(
            Solution::find_different_binary_string(vec![
                String::from("111"),
                String::from("011"),
                String::from("001"),
            ]),
            String::from("000")
        );
    }

    #[test]
    fn sample_4() {
        assert_eq!(
            Solution::find_different_binary_string(vec![
                String::from("001"),
                String::from("000"),
                String::from("010"),
            ]),
            String::from("011")
        );
    }
}