#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut stack = vec![];
        for ch in s.chars() {
            if ch.is_digit(10) {
                    stack.pop();
            } else {
                stack.push(ch as i32);
            }
        }
        stack.iter().map(|&x| x as u8 as char).collect()
    }
}