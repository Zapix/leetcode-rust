use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let mut sums = vec![];
        let mut sum = 0;
        for value in code.iter() {
            sum += value;
            sums.push(sum);
        }
        match k.cmp(&0) {
            Ordering::Equal => (0..code.len()).map(|_| 0).collect::<Vec<i32>>(),
            Ordering::Less => (0..code.len())
                .map(|x| {
                    if (x as i32 + k) > 0 {
                        sums[x - 1] - sums[(x as i32 + k - 1) as usize]
                    } else {
                        let rest = (x as i32 + k).abs() as usize;
                        *sums.last().unwrap() - sums[code.len() - rest - 1]
                            + if x > 0 { sums[x - 1] } else { 0 }
                    }
                })
                .collect::<Vec<i32>>(),
            Ordering::Greater => (0..code.len())
                .map(|x| {
                    if (x + k as usize) >= code.len() {
                        *sums.last().unwrap() - sums[x] + sums[(x + k as usize) % code.len()]
                    } else {
                        sums[x + k as usize] - sums[x]
                    }
                })
                .collect::<Vec<i32>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let code = vec![5, 7, 1, 4];
        let k = 3;
        let expected = vec![12, 10, 16, 13];
        assert_eq!(Solution::decrypt(code, k), expected);
    }

    #[test]
    fn test_example_2() {
        let code = vec![1, 2, 3, 4];
        let k = 0;
        let expected = vec![0, 0, 0, 0];
        assert_eq!(Solution::decrypt(code, k), expected);
    }

    #[test]
    fn test_example_3() {
        let code = vec![2, 4, 9, 3];
        let k = -2;
        let expected = vec![12, 5, 6, 13];
        assert_eq!(Solution::decrypt(code, k), expected);
    }
}
