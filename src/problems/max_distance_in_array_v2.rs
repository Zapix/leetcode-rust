use std::cmp::Reverse;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut min_value: Option<i32> = None;
        let mut max_value: Option<i32> = None;
        let mut distance= 0;

        for array in arrays.iter() {
            let current_min = *array.iter().next().unwrap();
            let current_max = *array.iter().last().unwrap();

            if min_value.is_none() {
                min_value = Some(current_min);
                max_value = Some(current_max);
                continue;
            }

            let current_distance = (max_value.unwrap() - current_min)
                .abs()
                .max(
                    (current_max - min_value.unwrap()).abs()
                );

            distance = distance.max(current_distance);
            min_value = Some(
                min_value.unwrap().min(current_min)
            );
            max_value = Some(
                max_value.unwrap().max(current_max)
            );
        }
        distance
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
