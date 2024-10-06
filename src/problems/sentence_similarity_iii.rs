#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn check_equals(
        origin: &Vec<&str>,
        source: &Vec<&str>,
        head1: usize,
        tail1: usize,
        head2: usize,
        tail2: usize,
    ) -> bool {
        if head2 == tail2 && (origin[head1] == source[head2] || origin[tail1] == source[head2]) {
            return true;
        }
        if origin[head1] == source[head2]
            && Solution::check_equals(origin, source, head1 + 1, tail1, head2 + 1, tail2)
        {
            return true;
        }
        if origin[tail1] == source[tail2]
            && Solution::check_equals(origin, source, head1, tail1 - 1, head2, tail2 - 1)
        {
            return true;
        }
        false
    }
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let (origin, source) = if sentence1.len() > sentence2.len() {
            (
                sentence1.split(" ").collect::<Vec<_>>(),
                sentence2.split(" ").collect::<Vec<_>>(),
            )
        } else {
            (
                sentence2.split(" ").collect::<Vec<_>>(),
                sentence1.split(" ").collect::<Vec<_>>(),
            )
        };
        Solution::check_equals(&origin, &source, 0, origin.len() - 1, 0, source.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let sentence1 = "My name is Haley".to_string();
        let sentence2 = "My Haley".to_string();
        let result = Solution::are_sentences_similar(sentence1, sentence2);
        assert_eq!(result, true);
    }

    #[test]
    fn leetcode_2() {
        let sentence1 = "of".to_string();
        let sentence2 = "A lot of words".to_string();
        let result = Solution::are_sentences_similar(sentence1, sentence2);
        assert_eq!(result, false);
    }

    #[test]
    fn leetcode_3() {
        let sentence1 = "Eating right now".to_string();
        let sentence2 = "Eating".to_string();
        let result = Solution::are_sentences_similar(sentence1, sentence2);
        assert_eq!(result, true);
    }

    #[test]
    fn leetcode_4() {
        let sentence1 = "My name is Haley".to_string();
        let sentence2 = "My name".to_string();
        let result = Solution::are_sentences_similar(sentence1, sentence2);
        assert_eq!(result, true);
    }

    #[test]
    fn leetcode_5() {
        let sentence1 = "a b c b b".to_string();
        let sentence2 = "a b b".to_string();
        let result = Solution::are_sentences_similar(sentence1, sentence2);
        assert_eq!(result, true);
    }

    #[test]
    fn leetcode_6() {
        let sentence1 = "a b b".to_string();
        let sentence2 = "a b b".to_string();
        let result = Solution::are_sentences_similar(sentence1, sentence2);
        assert_eq!(result, true);
    }

    fn leetcode_7() {
        let sentence1 = "a b c d e e d d e".to_string();
        let sentence2 = "a b c d e".to_string();
        let result = Solution::are_sentences_similar(sentence1, sentence2);
        assert_eq!(result, true);
    }
}
