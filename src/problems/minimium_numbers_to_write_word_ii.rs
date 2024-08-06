#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut char_counter = vec![0usize; 26];

        let a = 'a' as usize;
        for chr in word.chars() {
            char_counter[chr as usize - a] += 1;
        }
        char_counter.sort();

        let mut result= 0usize;

        for (i, cnt) in char_counter.iter().rev().enumerate() {
            result += cnt * (i / 8 + 1);
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::minimum_pushes(String::from("abcde")),
            5
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::minimum_pushes(String::from("xyzxyzxyzxyz")),
            12
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::minimum_pushes(String::from("aabbccddeeffgghhiiiiii")),
            24
        );
    }
}