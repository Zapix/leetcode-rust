struct Solution;

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let alphabet: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        let mut k = k;
        let mut increment = 0;
        while k > 0 {
            if k == 2i32.pow(k.ilog2()) {
                increment += k.ilog2() + 1;
                k = 0;
            } else {
                k = k - 2i32.pow(k.ilog2());
                increment += 1;
            }
        }
        alphabet[(increment - 1) as usize % 26]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_kth_character() {
        assert_eq!(Solution::kth_character(1), 'a');
        assert_eq!(Solution::kth_character(3), 'b');
        assert_eq!(Solution::kth_character(10), 'c');
        assert_eq!(Solution::kth_character(13), 'c');
        assert_eq!(Solution::kth_character(45), 'd');
    }
}
