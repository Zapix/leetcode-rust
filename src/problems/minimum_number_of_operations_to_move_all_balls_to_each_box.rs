#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut result = vec![0;boxes.len()];
        let mut score = 0;
        let mut count = 0;
        for (i, ch) in boxes.bytes().enumerate() {
            result[i] += score;
            if ch == '1' as u8 {
                count += 1;
            }
            score += count;
        }

        let mut score = 0;
        let mut count = 0;
        for (i, ch) in boxes.bytes().enumerate().rev() {
            result[i] += score;
            if ch == '1' as u8  {
                count += 1;
            }
            score += count;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::min_operations("110".to_string()),
            vec![1, 1, 3]
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::min_operations("001011".to_string()),
            vec![11, 8, 5, 4, 3, 4]
        );
    }

}