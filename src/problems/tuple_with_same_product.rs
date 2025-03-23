use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut product_map = HashMap::new();
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                let product = nums[i] * nums[j];
                let count = product_map.entry(product).or_insert(0);
                *count += 1;
            }
        }

        product_map.iter().fold(0, |acc, (_, &val)| {
            acc + if val > 1 {
                val * (val - 1) * 4
            } else {
                0
            }
        })
    }
}