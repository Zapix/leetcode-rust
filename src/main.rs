mod problems;
use std::cmp::max;

fn to_rev_vec_int(a: String) -> Vec<i32> {
    a.split("")
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .into_iter()
        .rev()
        .collect()
}

fn get_value(vec: &Vec<i32>, i: usize) -> i32 {
    match vec.get(i) {
        None => 0,
        Some(x) => *x
    }
}
fn add_binary(a: String, b: String) -> String {
    let a_vec= to_rev_vec_int(a);
    let a_len  = a_vec.len();
    let b_vec = to_rev_vec_int(b);
    let b_len = b_vec.len();
    let mut res_vec: Vec<i32> = Vec::new();
    let mut add_to = 0;

    let max_len = max(a_len, b_len);

    for i in 0..max_len {
        let a_digit = get_value(&a_vec, i);
        let b_digit = get_value(&b_vec, i);
        let digit_res = a_digit + b_digit + add_to;
        add_to = digit_res / 2;
        res_vec.push(digit_res % 2)
    }

    if add_to > 0 {
        res_vec.push(1);
    }

    res_vec
        .iter()
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn is_sub_string(needle: &String, haystack: &String, index: usize) -> bool {
    let length = needle.len();
    haystack.get(index..(index+length)) == needle.get(0..)
}

fn str_str(haystack: String, needle: String) -> i32 {
    if haystack.len() < needle.len() {
        return -1
    }
    let mut result = -1;
    let length = haystack.len() - needle.len() + 1;
    for i in 0..length {
        if is_sub_string(&needle, &haystack, i) {
            result = i as i32;
            break;
        }
    }
    result
}

fn main() {
    println!("Hello, world!");
    println!("{}", add_binary(String::from("1111"), String::from("1")));
    assert_eq!(add_binary(String::from("1111"), String::from("1")), "10000");
    assert_eq!(add_binary(String::from("11101111"), String::from("101")), "11110100");

    assert_eq!(str_str(String::from("sadbutsad"), String::from("sad")), 0);
    assert_eq!(str_str(String::from("butsad"), String::from("sad")), 3);
    assert_eq!(str_str(String::from("ssadbutsad"), String::from("sad")), 1);
    assert_eq!(str_str(String::from("leetcode"), String::from("leeto")), -1);
    assert_eq!(str_str(String::from("a"), String::from("a")), 0);
    assert_eq!(str_str(String::from("ba"), String::from("a")), 1);
    assert_eq!(str_str(String::from("a"), String::from("ab")), -1);
}
