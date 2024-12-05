#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let mut start_nodes = vec![];
        let mut target_nodes = vec![];

        for (l, t) in start.chars().zip(target.chars()) {
            match l {
                'L' => {
                    if !start_nodes.is_empty() && *start_nodes.last().unwrap() == 'R' {
                        return false;
                    }
                    start_nodes.push('L');
                }
                'R' => {
                    start_nodes.push('R');
                }
                _ => {}
            };
            match t {
                'L' => {
                    target_nodes.push('L');
                }
                'R' => {
                    if !target_nodes.is_empty() && *target_nodes.last().unwrap() == 'L' {
                        return false;
                    }
                    target_nodes.push('R');
                }
                _ => {}
            }
            if start_nodes.is_empty() && target_nodes.is_empty() {
                continue;
            }
            if !start_nodes.is_empty() && !target_nodes.is_empty() {
                if start_nodes.last().unwrap() != target_nodes.last().unwrap() {
                    return false;
                }
                start_nodes.pop();
                target_nodes.pop();
                continue;
            }
            if !start_nodes.is_empty() {
                if *start_nodes.last().unwrap() == 'L' {
                    return false;
                }
            }
            if !target_nodes.is_empty() {
                if *target_nodes.last().unwrap() == 'R' {
                    return false;
                }
            }
        }

        start_nodes.is_empty() && target_nodes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let start = String::from("_L__R__R_");
        let target = String::from("L______RR");
        assert_eq!(Solution::can_change(start, target), true);
    }

    #[test]
    fn test_example_2() {
        let start = String::from("R_L_");
        let target = String::from("__LR");
        assert_eq!(Solution::can_change(start, target), false);
    }

    #[test]
    fn test_example_3() {
        let start = String::from("_R");
        let target = String::from("R_");
        assert_eq!(Solution::can_change(start, target), false);
    }

    #[test]
    fn test_example_4() {
        let start = String::from("L_");
        let target = String::from("_L");
        assert_eq!(Solution::can_change(start, target), false);
    }

    #[test]
    fn test_example_5() {
        let start = String::from("__RL");
        let target = String::from("_LR_");
        assert_eq!(Solution::can_change(start, target), false);
    }
}
