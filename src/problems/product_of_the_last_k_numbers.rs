#[allow(dead_code)]
struct Solution {
    prefix_products: Vec<i32>,
}

#[allow(dead_code)]
impl Solution {
    fn new() -> Self {
        Self {
            prefix_products: vec![1],
        }
    }

    fn add(&mut self, num: i32) {
        if num != 0 {
            let last_product = *self.prefix_products.last().unwrap();
            self.prefix_products.push(last_product * num);
        } else {
            self.prefix_products = vec![1];
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let n = self.prefix_products.len();
        if k as usize >= n {
            return 0;
        }
        self.prefix_products[n - 1] / self.prefix_products[n - 1 - k as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_of_the_last_k_numbers() {
        let mut obj = Solution::new();
        obj.add(3);
        obj.add(0);
        obj.add(2);
        obj.add(5);
        obj.add(4);
        assert_eq!(obj.get_product(2), 20);
        assert_eq!(obj.get_product(3), 40);
        assert_eq!(obj.get_product(4), 0);
        obj.add(8);
        assert_eq!(obj.get_product(2), 32);
    }
}

