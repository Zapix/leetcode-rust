#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let chars = s.chars().collect::<Vec<_>>();
        let shift = shift.iter().map(|x| match x[0] {
            0 => x[1],
            1 => -x[1],
            _ => 0
        }).sum::<i32>();



        String::from_iter(
            (0..n)
                .map(|x| (((x as i32 + shift as i32) % n as i32 + n as i32) % n as i32) as usize)
                .map(|x| chars[x])
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::string_shift("abc".to_string(), vec![vec![0, 1], vec![1, 2]]), "cab".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::string_shift("abcdefg".to_string(), vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]]), "efgabcd".to_string());
    }
}