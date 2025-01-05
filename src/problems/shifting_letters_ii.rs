use std::collections::HashMap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn build_shifts(shifts: &Vec<Vec<i32>>) -> HashMap<usize, i32> {
        let mut shift_results = HashMap::new();

        for shift in shifts {
            let delta = match shift[2] {
                0 => -1,
                1 => 1,
                other => panic!("Un expected shift {other}"),
            };
            let start = usize::try_from(shift[0]).unwrap();
            let entry = shift_results.entry(start).or_insert(0);
            *entry = (*entry + delta + 26) % 26;

            let end = usize::try_from(shift[1] + 1).unwrap();
            let entry = shift_results.entry(end).or_insert(0);
            *entry = (*entry - delta + 26) % 26;
        }

        shift_results
    }
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let shifts = Solution::build_shifts(&shifts);
        let mut result = vec![];
        let mut delta = 0;
        for (i, ch) in s.chars().enumerate() {
            delta = (delta + shifts.get(&i).unwrap_or(&0)) % 26;
            let val = ((ch as u8 - 'a' as u8) + delta as u8) % 26;
            result.push(('a' as u8 + val) as char);
        }
        String::from_iter(result.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("abc");
        let shifts = vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]];
        assert_eq!(Solution::shifting_letters(s, shifts), "ace");
    }

    #[test]
    fn test_example_2() {
        let s = String::from("dztz");
        let shifts = vec![vec![0, 0, 0], vec![1, 1, 1]];
        assert_eq!(Solution::shifting_letters(s, shifts), "catz");
    }
}
