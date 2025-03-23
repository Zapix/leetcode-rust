struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let first_char = s.clone().chars().take(1).last().unwrap();
        let shifts = goal
            .clone()
            .chars()
            .enumerate()
            .filter_map(|x| if x.1 == first_char { Some(x.0) } else { None })
            .collect::<Vec<usize>>();

        for shift in shifts {
            let matches = s
                .clone()
                .chars()
                .zip(
                    goal.clone()
                        .chars()
                        .skip(shift)
                        .chain(goal.clone().chars().take(shift)),
                )
                .all(|x| x.0 == x.1);

            if matches {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_rotate_string_example1() {
        let s = String::from("abcde");
        let goal = String::from("cdeab");
        assert_eq!(Solution::rotate_string(s, goal), true);
    }

    #[test]
    fn test_rotate_string_example2() {
        let s = String::from("abcde");
        let goal = String::from("abced");
        assert_eq!(Solution::rotate_string(s, goal), false);
    }

    #[test]
    fn test_rotate_string_example3() {
        let s = String::from("defdefdefabcabc");
        let goal = String::from("defdefabcabcdef");
        assert_eq!(Solution::rotate_string(s, goal), true);
    }
}
