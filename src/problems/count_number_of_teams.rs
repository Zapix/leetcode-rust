#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn num_teams(rating: Vec<i32>) -> i32 {
        let mut count_numbers_lesser: Vec<i32> = Vec::new();
        let mut count_numbers_greater: Vec<i32> = Vec::new();
        for i in 0..rating.len() {
            let mut count_lesser = 0;
            let mut count_greater = 0;
            for j in 0..i {
                if rating[i] > rating[j] {
                    count_lesser += 1;
                } else {
                    count_greater += 1;
                }
            }
            count_numbers_lesser.push(count_lesser);
            count_numbers_greater.push(count_greater);
        }
        let mut count_decreasing_teams: Vec<i32> = Vec::new();
        let mut count_increasing_teams: Vec<i32> = Vec::new();

        for i in 0..rating.len() {
            let mut count_increase_team = 0;
            let mut count_decrease_team = 0;
            for j in 0..i {
                if rating[i] > rating[j] {
                    count_increase_team += count_numbers_lesser[j];
                } else {
                    count_decrease_team += count_numbers_greater[j];
                }
            }
            count_increasing_teams.push(count_increase_team);
            count_decreasing_teams.push(count_decrease_team);
        }

        count_decreasing_teams.iter().sum::<i32>() + count_increasing_teams.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::num_teams(vec![2, 5, 3, 4, 1]),
            3,
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::num_teams(vec![2, 1, 4]),
            0
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::num_teams(vec![1, 2, 3, 4]),
            4
        )
    }
}