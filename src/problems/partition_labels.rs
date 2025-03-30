struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last_chars = [0; 26];
        for (i, ch) in s.chars().enumerate() {
            let idx = ch as usize - 'a' as usize;
            last_chars[idx] = i;
        }

        let mut result = vec![];
        let mut last_idx: Option<usize> = None;
        let mut cnt = 0;
        for (i, ch) in s.chars().enumerate() {
            cnt += 1;
            last_idx = match last_idx {
                Some(last_idx) if last_idx == i => {
                    result.push(cnt);
                    cnt = 0;
                    None
                }
                Some(last_idx) => {
                    let idx = ch as usize - 'a' as usize;
                    Some(last_idx.max(last_chars[idx]))
                }
                None => {
                    let idx = ch as usize - 'a' as usize;
                    if last_chars[idx] == i {
                        result.push(cnt);
                        cnt = 0;
                        None
                    } else {
                        Some(last_chars[idx])
                    }
                }
            }
        }

        if last_idx.is_some() {
            result.push(cnt);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("ababcbacadefegdehijhklij");
        assert_eq!(Solution::partition_labels(s), vec![9, 7, 8]);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("eccbbbbdec");
        assert_eq!(Solution::partition_labels(s), vec![10]);
    }
}
