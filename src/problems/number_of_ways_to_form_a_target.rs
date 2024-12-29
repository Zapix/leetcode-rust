#[allow(dead_code)]
struct Solution;

const MODULO: usize = 1_000_000_007;

#[allow(dead_code)]
impl Solution {
    fn build_chars_frequency(words: &Vec<String>) -> Vec<Vec<usize>> {
        let mut chars_frequency = vec![vec![0; 26]; words[0].len()];
        for word in words {
            for (i, ch) in word.chars().enumerate() {
                let ch_num = ch as u8 - 'a' as u8;
                let ch_num = ch_num as usize;
                chars_frequency[i][ch_num] += 1;
            }
        }
        chars_frequency
    }

    fn get_words(words: &Vec<String>, target: &String, words_index: usize, target_index: usize, dp: &mut Vec<Vec<Option<usize>>>, chars_frequency: &Vec<Vec<usize>>) -> usize {
        if target_index == target.len() {
            return 1;
        }
        if words_index == words[0].len() || words[0].len() - words_index < target.len() - target_index {
            return 0
        }
        if let Some(value) = dp[words_index][target_index] {
            return value;
        }

        let mut count_ways = 0;
        let cur_pos = target.chars().collect::<Vec<_>>()[target_index] as u8 - 'a' as u8;
        let cur_pos = cur_pos as usize;
        count_ways += Self::get_words(words, target, words_index + 1, target_index, dp, chars_frequency);
        count_ways += chars_frequency[words_index][cur_pos] *  Self::get_words(words, target, words_index + 1, target_index + 1, dp, chars_frequency);
        dp[words_index][target_index] = Some(count_ways % MODULO);
        dp[words_index][target_index].unwrap()
    }

    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let chars_frequency = Solution::build_chars_frequency(&words);
        let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; target.len()]; words[0].len()];
        Self::get_words(&words, &target, 0, 0, &mut dp, &chars_frequency) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_1() {
        let words = vec!["acca".to_string(),"bbbb".to_string(),"caca".to_string()];
        let target = "aba".to_string();
        assert_eq!(
            Solution::num_ways(words, target),
            6
        );
    }

    #[test]
    fn test_leetcode_2() {
        let words = vec!["abba".to_string(), "baab".to_string()];
        let target = "bab".to_string();
        assert_eq!(
            Solution::num_ways(words, target),
            4
        );
    }
}