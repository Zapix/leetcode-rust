const ARR_SIZE: usize = 10001;

#[allow(long_running_const_eval)]
const SYMMETRIC_VALUES: [bool; ARR_SIZE] = {
    let mut arr_size = [false; ARR_SIZE];
    let mut i = 0;

    while i < ARR_SIZE {
        let mut count = 0;
        let mut digits = [0; 10];
        let mut value = i;
        while value > 0 {
            digits[count] = value % 10;
            value /= 10;
            count += 1;
        }
        if count % 2 ==  0 {
            let mut sum1 = 0;
            let mut sum2 = 0;
            let mut j = 0;
            while j < count / 2 {
                sum1 += digits[j];
                sum2 += digits[count - j - 1];
                j += 1;
            }
            arr_size[i] = sum1 == sum2;

        }
        i += 1;
    }
    arr_size
};

struct Solution;

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        (low..(high + 1)).filter(|i| SYMMETRIC_VALUES[*i as usize]).count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let low = 1;
        let high = 100;
        let result = Solution::count_symmetric_integers(low, high);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_example_2() {
        let low = 1200;
        let high = 1230;
        let result = Solution::count_symmetric_integers(low, high);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_single_value_symmetric() {
        let low = 11;
        let high = 11;
        let result = Solution::count_symmetric_integers(low, high);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_single_value_non_symmetric() {
        let low = 10;
        let high = 10;
        let result = Solution::count_symmetric_integers(low, high);
        assert_eq!(result, 0);
    }
}
