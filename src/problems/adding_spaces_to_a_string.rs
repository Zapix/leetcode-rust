#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut space = spaces.iter();
        let mut value = space.next();
        let mut result = Vec::new();
        for (i, char) in s.chars().enumerate() {
            match value {
                Some(j) if *j == i as i32 => {
                    result.push(' ');
                    value = space.next();
                }
                _ => {}
            }
            result.push(char);
        }
        String::from_iter(result)
    }
}
