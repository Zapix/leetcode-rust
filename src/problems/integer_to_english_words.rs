use std::cell::{Ref, RefCell};
use std::collections::HashMap;
#[allow(dead_code)]
struct Solution;

#[allow(daed_code)]
impl Solution {

    fn by_digits(num: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut num = num;
        result.push(num % 10);
        num = num / 10;
        while num > 0 {
            result.push(num % 10);
            num = num / 10;
        }
        result
    }

    fn by_group_of_3(digits: Vec<i32>) -> Vec<RefCell<Vec<i32>>> {
        let mut result: Vec<RefCell<Vec<i32>>> = Vec::new();
        for (i, digit) in digits.iter().enumerate() {
           if i % 3 == 0 {
               result.push(RefCell::new(Vec::new()));
           }
            let group = result[i / 3].get_mut();
            group.push(*digit);
        }
        result
    }

    fn get_digit_word(value: &i32) -> String {
        let digits: HashMap<i32, &str> = HashMap::from([
            (1, "One"),
            (2, "Two"),
            (3, "Three"),
            (4, "Four"),
            (5, "Five"),
            (6, "Six"),
            (7, "Seven"),
            (8, "Eight"),
            (9, "Nine"),
        ]);
        String::from(*digits.get(value).unwrap())
    }

    fn get_teen_word(value: &i32) -> String {
        let teens: HashMap<i32, &str> = HashMap::from([
            (10, "Ten"),
            (11, "Eleven"),
            (12, "Twelve"),
            (13, "Thirteen"),
            (14, "Fourteen"),
            (15, "Fifteen"),
            (16, "Sixteen"),
            (17, "Seventeen"),
            (18, "Eighteen"),
            (19, "Nineteen"),
        ]);

        String::from(*teens.get(value).unwrap())
    }

    fn get_tens_word(value: &i32) -> String {
        let tens: HashMap<i32, &str> = HashMap::from([
            (20, "Twenty"),
            (30, "Thirty"),
            (40, "Forty"),
            (50, "Fifty"),
            (60, "Sixty"),
            (70, "Seventy"),
            (80, "Eighty"),
            (90, "Ninety"),
        ]);
        String::from(*tens.get(value).unwrap())
    }

    fn group_by_3_to_words(group: Ref<Vec<i32>>) -> String {

        let mut result: Vec<String> = Vec::new();

        match group.get(2) {
            Some(x) if *x > 0 => {
                result.push(Self::get_digit_word(x));
                result.push(String::from("Hundred"));
            }
            _ => {}
        }

        match group.get(1).or(Some(&0)) {
            Some(x) if *x == 0 => {
                match group.get(0).or(Some(&0)) {
                    Some(x) if *x == 0 => {},
                    Some(x) if *x < 10 => {
                        result.push(Self::get_digit_word(x));
                    },
                    _ => {}
                }
            }
            Some(x) if *x == 1 => {
                let number = 10 + *group.get(0).or(Some(&0)).unwrap();
                result.push(Self::get_teen_word(&number))
            }
            Some(x) if *x < 10 => {
                let number = *x * 10;
                result.push(Self::get_tens_word(&number));
                match group.get(0).or(Some(&0)) {
                    Some(x) if *x == 0 => {},
                    Some(x) if *x < 10 => {
                        result.push(Self::get_digit_word(x));
                    },
                    _ => {}
                }
            }
            _ => {}
        }

        result.join(" ")
    }

    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return String::from("Zero");
        }

        let digits = Self::by_digits(num);
        let groups = Self::by_group_of_3(digits);
        let mut result: Vec<String> = Vec::new();
        for (i, group_ref) in groups.iter().enumerate() {
            let group = group_ref.borrow();
            let word = Self::group_by_3_to_words(group);
            if word.is_empty() {
                continue
            }

            match i {
                1 => {
                    result.push(String::from("Thousand"));
                }
                2 => {
                    result.push(String::from("Million"));
                }
                3 => {
                    result.push(String::from("Billion"));
                }
                _ => {}
            }

            result.push(word)
        }
        result.reverse();
        result.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::number_to_words(123),
            String::from("One Hundred Twenty Three"),
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::number_to_words(12345),
            String::from("Twelve Thousand Three Hundred Forty Five")
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::number_to_words(1234567),
            String::from("One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven")
        );
    }

    #[test]
    fn leetcode_4() {
        assert_eq!(
            Solution::number_to_words(2147483646),
            String::from("Two Billion One Hundred Forty Seven Million Four Hundred Eighty Three Thousand Six Hundred Forty Six")
        )
    }

    #[test]
    fn leetcode_5() {
        assert_eq!(
            Solution::number_to_words(1000),
            String::from("One Thousand")
        )
    }

    #[test]
    fn leetcode_6() {
        assert_eq!(
            Solution::number_to_words(1000000),
            String::from("One Million")
        )
    }

}