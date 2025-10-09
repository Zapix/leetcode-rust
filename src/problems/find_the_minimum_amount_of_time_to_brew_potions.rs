struct Solution;

impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let mut prev_times = vec![0i64; skill.len() + 1];
        let mut current_times = vec![0i64; skill.len() + 1];

        for (_, &m) in mana.iter().enumerate() {
            for j in (0..=skill.len()).rev() {
                if (j == skill.len()) {
                    current_times[j] = prev_times[j] + (m * skill[j - 1]) as i64;
                } else {
                    current_times[j] =
                        prev_times[j + 1].max(current_times[j + 1] - (m * skill[j]) as i64);
                }
            }
            prev_times[0] = current_times[0];
            for j in 0..skill.len() {
                prev_times[j + 1] = prev_times[j] + (m * skill[j]) as i64;
            }
            println!("{:?}", prev_times);
        }
        prev_times[skill.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_time_1() {
        let skill = vec![1, 5, 2, 4];
        let mana = vec![5, 1, 4, 2];

        assert_eq!(Solution::min_time(skill, mana), 110);
    }

    #[test]
    fn test_min_time_2() {
        let skill = vec![1, 1, 1];
        let mana = vec![1, 1, 1];
        assert_eq!(Solution::min_time(skill, mana), 5);
    }

    #[test]
    fn test_min_time_3() {
        let skill = vec![1, 2, 3, 4];
        let mana = vec![1, 2];
        assert_eq!(Solution::min_time(skill, mana), 21);
    }
}
