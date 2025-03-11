struct Solution;

impl Solution {
    pub fn compute_amount(size: usize, x: Option<usize>, y: Option<usize>) -> i32 {
        match (x, y) {
            (Some(x), Some(y)) => (size - x.max(y)) as i32,
            _ => 0,
        }
    }
    pub fn number_of_substrings(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<_>>();
        let mut next_a = None;
        let mut next_b = None;
        let mut next_c = None;
        let mut a_idxs = vec![None; chars.len()];
        let mut b_idxs = vec![None; chars.len()];
        let mut c_idxs = vec![None; chars.len()];
        for i in (0..chars.len()).rev() {
            match chars[i] {
                'a' => {
                    next_a = Some(i);
                }
                'b' => {
                    next_b = Some(i);
                }
                'c' => {
                    next_c = Some(i);
                }
                _ => {}
            }

            a_idxs[i] = next_a;
            b_idxs[i] = next_b;
            c_idxs[i] = next_c;
        }
        let mut result = 0;
        for i in (0..chars.len()) {
            result += match chars[i] {
                'a' => Solution::compute_amount(chars.len(), b_idxs[i], c_idxs[i]),
                'b' => Solution::compute_amount(chars.len(), a_idxs[i], c_idxs[i]),
                'c' => Solution::compute_amount(chars.len(), b_idxs[i], a_idxs[i]),
                _ => 0,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("abcabc");
        assert_eq!(Solution::number_of_substrings(s), 10);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("aaacb");
        assert_eq!(Solution::number_of_substrings(s), 3);
    }

    #[test]
    fn test_example_3() {
        let s = String::from("abc");
        assert_eq!(Solution::number_of_substrings(s), 1);
    }
}
