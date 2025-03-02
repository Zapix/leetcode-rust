#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut j = 0;
        let mut result = vec![];

        while i < nums1.len() && j < nums2.len() {
            if nums1[i][0] == nums2[j][0] {
                result.push(vec![nums1[i][0], nums1[i][1] + nums2[j][1]]);
                i += 1;
                j += 1;
            } else if nums1[i][0] < nums2[j][0] {
                result.push(vec![nums1[i][0], nums1[i][1]]);
                i += 1;
            } else {
                result.push(vec![nums2[j][0], nums2[j][1]]);
                j += 1;
            }
        }

        if i == nums1.len() {
            result.extend(nums2.into_iter().skip(j));
        } else {
            result.extend(nums1.into_iter().skip(i));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums1 = vec![vec![1, 2], vec![2, 3], vec![4, 5]];
        let nums2 = vec![vec![1, 4], vec![3, 2], vec![4, 1]];
        assert_eq!(
            Solution::merge_arrays(nums1, nums2),
            vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]]
        );
    }

    #[test]
    fn test_example_2() {
        let nums1 = vec![vec![2, 4], vec![3, 6], vec![5, 5]];
        let nums2 = vec![vec![1, 3], vec![4, 3]];
        assert_eq!(
            Solution::merge_arrays(nums1, nums2),
            vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]]
        );
    }
}
