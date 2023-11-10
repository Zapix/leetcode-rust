pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut common: String = strs.get(0).unwrap().clone();
        for str in strs {
            common = String::from(
                common
                    .chars()
                    .zip(str.chars())
                    .take_while(|(a, b)| a == b)
                    .map(|x| String::from(x.0))
                    .collect::<Vec<String>>()
                    .join("")
            );
        }
        common
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight"),
            ]),
            String::from("fl")
        );
    }

    #[test]
    fn simple_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ]),
            String::from("")
        );
    }

    #[test]
    fn simple_3() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("cir"),
                String::from("car")
            ]),
            String::from("c")
        );
    }
}