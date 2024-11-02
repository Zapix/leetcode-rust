#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words = sentence.split(' ');
        words
            .clone()
            .zip(words.clone().skip(1).chain(words.clone().take(1)))
            .all(|(a, b)| a.chars().last() == b.chars().take(1).last())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_circular_sentence_case1() {
        let sentence = String::from("leetcode exercises sound delightful");
        assert_eq!(Solution::is_circular_sentence(sentence), true);
    }

    #[test]
    fn test_is_circular_sentence_case2() {
        let sentence = String::from("eetcode");
        assert_eq!(Solution::is_circular_sentence(sentence), true);
    }

    #[test]
    fn test_is_circular_sentence_case3() {
        let sentence = String::from("Leetcode is cool");
        assert_eq!(Solution::is_circular_sentence(sentence), false);
    }
}
