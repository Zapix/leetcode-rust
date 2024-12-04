#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let mut iter1 = str1.chars();
        let mut iter2 = str2.chars();
        let mut c1 = iter1.next();
        let mut c2 = iter2.next();

        while c1.is_some() && c2.is_some() {
            if c1.unwrap() == c2.unwrap() {
                c2 = iter2.next();
                c1 = iter1.next();
                continue;
            }

            if ((c1.unwrap() as u8 + 1) - ('a' as u8)) % 26 == (c2.unwrap() as u8) - ('a' as u8) {
                c2 = iter2.next();
                c1 = iter1.next();
                continue;
            }

            c1 = iter1.next();
        }

        c2.is_none()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::can_make_subsequence("abc".to_string(), "ad".to_string()), true);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::can_make_subsequence("zc".to_string(), "ad".to_string()), true);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::can_make_subsequence("ab".to_string(), "d".to_string()), false);
    }
}