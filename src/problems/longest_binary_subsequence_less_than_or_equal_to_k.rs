struct Solution;

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let k_len = (k.checked_ilog2().unwrap() + 1) as usize;
        let mut i = 0;
        let mut value = 0;
        while i < k_len && i < s.len() {
            let idx = s.len() - i - 1;
            value += (s[idx] as u8 - '0' as u8) as i32 * 2i32.pow(i as u32);
            if value > k {
                break;
            }
            i += 1;
        }
        let mut count = 0;
        for j in 0..(s.len() - i) {
            if s[j] == '0' {
                count += 1;
            }
        }
        (i + count) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let s = "1001010".to_string();
        let k = 5;
        let result = Solution::longest_subsequence(s, k);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_example2() {
        let s = "00101001".to_string();
        let k = 1;
        let result = Solution::longest_subsequence(s, k);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_example3() {
        let s  = "111100010000011101001110001111000000001011101111111110111000011111011000010101110100110110001111001001011001010011010000011111101001101000000101101001110110000111101011000101".to_string();
        let k = 11713332;
        let result = Solution::longest_subsequence(s, k);
        assert_eq!(result, 96)
    }
}
