pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let length = s.len();
        for i in 0..(length / 2) {
            let buf = s[i];
            s[i] = s[length - 1 - i];
            s[length - 1 - i] = buf;
        }
    }
}

#[cfg(test)]
mod tests {
    use super ::*;

    #[test]
    fn simple_1() {
        let str: &mut Vec<char> = &mut vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(str);
        assert_eq!(*str, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn simple_2() {
        let str: &mut Vec<char> = &mut vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string(str);
        assert_eq!(*str, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}
