#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }
        let mut opened_count = 0;
        let mut any_count = 0;
        for (ch, lock) in s.chars().zip(locked.chars()) {
            let closed = ch == ')';
            let locked = lock == '1';
            match (locked, closed) {
                (true, true) => {
                    if opened_count == 0 && any_count == 0 {
                        return false;
                    }
                    if opened_count > 0 {
                        opened_count -= 1;
                    } else {
                        any_count -= 1;
                    }
                }
                (true, false) => {
                    opened_count += 1;
                }
                _ => {
                    any_count += 1;
                }
            }
        }

        let mut closed_count = 0;
        let mut any_count = 0;
        for (ch, lock) in s.chars().rev().zip(locked.chars().rev()) {
            let opened = '(' == ch;
            let locked = lock == '1';
            match (locked, opened) {
                (true, true) => {
                    if closed_count == 0 && any_count == 0 {
                        return false;
                    }
                    if closed_count > 0 {
                        closed_count -= 1;
                    } else {
                        any_count -= 1;
                    }
                }
                (true, false) => {
                    closed_count += 1;
                }
                _ => {
                    any_count += 1;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("))()))");
        let locked = String::from("010100");
        assert_eq!(Solution::can_be_valid(s, locked), true);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("()()");
        let locked = String::from("0000");
        assert_eq!(Solution::can_be_valid(s, locked), true);
    }

    #[test]
    fn test_example_3() {
        let s = String::from(")");
        let locked = String::from("0");
        assert_eq!(Solution::can_be_valid(s, locked), false);
    }
}
