use std::cmp::Reverse;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut arrays_min = arrays.iter().enumerate().map(|(i, array)| {
            let min = *array.iter().next().unwrap();
            (i, min)
        }).collect::<Vec<(usize, i32)>>();
        arrays_min.sort_by_key(|a| a.1);

        let mut arrays_max = arrays.iter().enumerate().map(|(i, array)| {
            let max = *array.iter().last().unwrap();
            (i, max)
        }).collect::<Vec<(usize, i32)>>();
        arrays_max.sort_by_key(|a| Reverse(a.1));

        if arrays_min[0].0 != arrays_max[0].0 {
            return arrays_max[0].1 - arrays_min[0].1;
        }

        (arrays_max[0].1 - arrays_min[1].1).max(arrays_max[1].1 - arrays_min[0].1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::max_distance(
                vec![
                    vec![1, 2, 3],
                    vec![4, 5],
                    vec![1, 2, 3]
                ]
            ),
            4
        );
    }

    #[test]
    fn  leetcode_2() {
        assert_eq!(
            Solution::max_distance(
                vec![
                    vec![1],
                    vec![1]
                ]
            ),
            0
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::max_distance(
                vec![
                    vec![1, 4],
                    vec![0, 5]
                ]
            ),
            4
        );
    }
}