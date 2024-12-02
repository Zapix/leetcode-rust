#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence
            .split(' ')
            .enumerate()
            .find_map(|x| {
                if x.1.starts_with(search_word.as_str()) {
                    Some(x.0 as i32)
                } else {
                    None
                }
            })
            .unwrap_or(-1)
    }
}
