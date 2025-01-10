#[allow(dead_code)]
struct Solution;

struct Counter([usize; 26]);

impl Counter {
    pub fn parse(s: &str) -> Self {
        let mut result = [0; 26];
        for ch in s.chars() {
            let idx = (ch as u8 - 'a' as u8) as usize;
            result[idx] += 1;
        }
        Self(result)
    }

    pub fn empty() -> Self {
        Self([0; 26])
    }

    pub fn merge_subsets(a: &Counter, b: &Counter) -> Self {
        let mut result = [0; 26];
        for i in 0..26 {
            result[i] = if a.0[i] > b.0[i] {
                a.0[i]
            } else {
                b.0[i]
            }
        }
        Self(result)
    }

    pub fn is_subset(&self, val: &Counter) -> bool {
        val.0.iter().zip(self.0.iter()).all(|x| x.0 >= x.1)
    }

}

#[allow(dead_code)]
impl Solution {

    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let super_set = words2
            .iter()
            .map(|x| Counter::parse(x.as_str()))
            .fold(
                Counter::empty(),
                |acc, x| Counter::merge_subsets(&acc, &x)
            );
        words1
            .into_iter()
            .filter(|val| {
                let val = Counter::parse(val.as_str());
                super_set.is_subset(&val)
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_subsets_example1() {
        let words1 = vec![
            "amazon".to_string(),
            "apple".to_string(),
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let words2 = vec!["e".to_string(), "o".to_string()];
        let result = Solution::word_subsets(words1, words2);
        assert_eq!(result, vec!["facebook".to_string(), "google".to_string(), "leetcode".to_string()]);
    }

    #[test]
    fn test_word_subsets_example2() {
        let words1 = vec![
            "amazon".to_string(),
            "apple".to_string(),
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let words2 = vec!["l".to_string(), "e".to_string()];
        let result = Solution::word_subsets(words1, words2);
        assert_eq!(result, vec!["apple".to_string(), "google".to_string(), "leetcode".to_string()]);
    }
}

