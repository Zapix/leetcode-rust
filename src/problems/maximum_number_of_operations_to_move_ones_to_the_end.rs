struct Solution;

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut arr = s
            .chars()
            .map(|c| (c as u8 - '0' as u8) as i32)
            .collect::<Vec<i32>>();
        arr.push(1);
        let arr = arr;
        let mut result = 0i32;
        let mut c1 = arr[0];

        for i in 1..arr.len() {
            if arr[i - 1] == 0 && arr[i] == 1 {
                result += c1;
            }
            if arr[i] == 1 {
                c1 += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_operations_1() {
        assert_eq!(Solution::max_operations("1001101".to_string()), 4)
    }

    #[test]
    fn test_max_operations_2() {
        assert_eq!(Solution::max_operations("00111".to_string()), 0)
    }
}
