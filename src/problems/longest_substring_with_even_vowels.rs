struct Solution;

impl Solution {
    fn get_ch_bit(ch: char) -> i32 {
        match ch {
            'a' => 1,
            'e' => 1 << 1,
            'i' => 1 << 2,
            'o' => 1 << 3,
            'u' => 1 << 4,
            _ => 0,
        }
    }
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut prefix_xor = 0;
        let mut mp = [-1; 32];
        let mut longest = 0;
        for (i, ch) in s.chars().enumerate() {
            prefix_xor ^= Solution::get_ch_bit(ch);
            if mp[prefix_xor as usize] == -1 && prefix_xor != 0 {
                mp[prefix_xor as usize] = i as i32
            }
            longest = longest.max(i as i32 - mp[prefix_xor as usize]);
        }
        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode1() {
        assert_eq!(
            Solution::find_the_longest_substring("eleetminicoworoep".to_string()),
            13
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::find_the_longest_substring("leetcodeisgreat".to_string()),
            5
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::find_the_longest_substring("bcbcbc".to_string()),
            6
        );
    }
}
