#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .map(|x| (*x)[11..13].to_string().parse::<i32>().unwrap())
            .filter(|x| *x > 60)
            .count() as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::count_seniors(
                vec![
                    String::from("7868190130M7522"),
                    String::from("5303914400F9211"),
                    String::from("9273338290F4010"),
                ]
            ),
            2
        )
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::count_seniors(
                vec![
                    String::from("1313579440F2036"),
                    String::from("2921522980M5644"),
                ]
            ),
            0
        )
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::count_seniors(
                vec![
                    String::from("7868190130M7522"),
                    String::from("5303914400F9211"),
                    String::from("9273338290F6110"),
                ]
            ),
            3
        )
    }
}