#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn update_bits_count(bits_count: &mut [i32; 32], val: i32) {
        for i in 0..32 {
            if val & (1 << i) > 0 {
                bits_count[i] += 1;
            }
        }
    }

    fn remove_bits_count(bits_count: &mut [i32; 32], val: i32) {
        for i in 0..32 {
            if val & (1 << i) > 0 {
                bits_count[i] -= 1;
            }
        }
    }

    fn bits_count_without(bits_count: &[i32; 32], val: i32) -> [i32; 32] {
        let mut new_bits_count = [0; 32];
        for i in 0..32 {
            new_bits_count[i] = bits_count[i];
            if val & (1 << i) > 0 {
                new_bits_count[i] -= 1;
            }
        }

        new_bits_count
    }

    fn number_from_bits(bits_count: &[i32; 32]) -> i32 {
        let mut result = 0;
        for i in 0..32 {
            if bits_count[i] > 0 {
                result |= 1 << i;
            }
        }
        result
    }

    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut bits_count = [0i32; 32];
        let mut j = 0;
        let mut min_length = i32::MAX;

        for i in 0..nums.len() {
            Solution::update_bits_count(&mut bits_count, nums[i]);
            if Self::number_from_bits(&bits_count) >= k {
                min_length = min_length.min((i - j + 1) as i32);
                while j <= i
                    && Self::number_from_bits(&Self::bits_count_without(&bits_count, nums[j])) >= k
                {
                    Self::remove_bits_count(&mut bits_count, nums[j]);
                    j += 1;
                }
                if (j <= i) {
                    min_length = min_length.min((i - j + 1) as i32);
                }
            }
        }

        if min_length == i32::MAX {
            -1
        } else {
            min_length
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3)
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::minimum_subarray_length(vec![2, 1], 0), 1)
    }
}
