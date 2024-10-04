#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut skill = skill;
        let n = skill.len();
        skill.sort();
        let expected = skill.first().unwrap() + skill.last().unwrap();
        (0..(n / 2))
            .map(|i| {
                if skill[i] + skill[n - 1 - i] == expected {
                    Some(skill[i] as i64 * skill[n - i - 1] as i64)
                } else {
                    None
                }
            })
            .fold(Some(0i64), |acc, item| {
                if acc.is_some() && item.is_some() {
                    Some(acc.unwrap() + item.unwrap())
                } else {
                    None
                }
            })
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_players() {
        let skill = vec![3, 2, 5, 1, 3, 4];
        let result = Solution::divide_players(skill);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_divide_players_small() {
        let skill = vec![3, 4];
        let result = Solution::divide_players(skill);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_divide_players_invalid() {
        let skill = vec![1, 1, 2, 3];
        let result = Solution::divide_players(skill);
        assert_eq!(result, -1);
    }
}
