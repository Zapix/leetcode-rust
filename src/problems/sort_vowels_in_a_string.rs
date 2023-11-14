#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {

    pub fn sort_vowels(s: String) -> String {
        let vowels = ['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];
        let mut string_vowels: Vec<char> = Vec::new();
        for c in s.chars() {
            if vowels.contains(&c) {
                string_vowels.push(c);
            }
        }
        string_vowels.sort();
        string_vowels.reverse();

        let mut sorted_string: Vec<char> = Vec::new();
        for c in s.chars() {
            if vowels.contains(&c) {
                sorted_string.push(string_vowels.pop().unwrap());
            } else {
                sorted_string.push(c);
            }
        }

        String::from(
            sorted_string
                .iter()
                .map(|x| String::from(*x))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_1() {
        assert_eq!(
            Solution::sort_vowels(String::from("Codewars")),
            String::from("Cadewors")
        );
    }

    #[test]
    fn simple_2() {
        assert_eq!(
            Solution::sort_vowels(String::from("leetcOdE")),
            String::from("lEOtcede")
        );
    }
}
