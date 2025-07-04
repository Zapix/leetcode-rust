struct Solution;

impl Solution {
    pub fn kth_character(mut k: i64, mut operations: Vec<i32>) -> char {
        let mut k: i128 = k as i128;
        let alphabet: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let mut increment = 0;

        while operations.len() > 0 {
            let operations_len = operations.len();
            if 2i128.pow(operations_len as u32 - 1) >= k {
                operations.pop();
                continue;
            }
            match operations.pop() {
                Some(0) => {
                    k -= 2i128.pow(operations_len as u32 - 1);
                }
                Some(1) => {
                    increment += 1;
                    k -= 2i128.pow(operations_len as u32 - 1);
                }
                _ => break,
            }
        }
        alphabet[increment as usize % 26]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::kth_character(5, vec![0, 0, 0]), 'a');
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::kth_character(10, vec![0, 1, 0, 1]), 'b');
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::kth_character(11, vec![0, 1, 0, 1]), 'c');
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::kth_character(15, vec![0, 1, 0, 1]), 'c');
    }

    #[test]
    fn example5() {
        assert_eq!(
            Solution::kth_character(
                33354182522397,
                vec![
                    0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0,
                    1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0,
                    0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0,
                    0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0
                ]
            ),
            'k'
        );
    }
}
