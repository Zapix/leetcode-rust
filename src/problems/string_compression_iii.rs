#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn by_nines(value: usize) -> Vec<usize> {
        let mut value = value;
        let mut result = vec![];
        while value > 9 {
            result.push(9usize);
            value -= 9
        }
        if value > 0 {
            result.push(value)
        }
        result
    }
    pub fn compressed_string(word: String) -> String {
        word.chars()
            .collect::<Vec<char>>()
            .chunk_by(|a, b| *a == *b)
            .map(|chunk| {
                Solution::by_nines(chunk.len())
                    .iter()
                    .map(|x| format!("{}{}", x, chunk[0]))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<String>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_compressed_string_example1() {
        assert_eq!(
            Solution::compressed_string("abcde".to_string()),
            "1a1b1c1d1e"
        );
    }

    #[test]
    fn test_compressed_string_example2() {
        assert_eq!(
            Solution::compressed_string("aaaaaaaaaaaaaabb".to_string()),
            "9a5a2b"
        );
    }
}
