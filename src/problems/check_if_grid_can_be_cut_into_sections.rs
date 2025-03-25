struct Solution;

const OPENED: i32 = 1;
const CLOSED: i32 = 0;
impl Solution {
    fn check_can_cut(n: i32, intervals: impl Iterator<Item = (i32, i32)>) -> bool {
        let mut count_sections = 0;
        let mut count_opened = 0;
        let mut nodes = intervals
            .map(|x| vec![(x.0, OPENED), (x.1, CLOSED)])
            .flatten()
            .collect::<Vec<_>>();
        nodes.sort();
        let nodes = nodes;
        for (_point, status) in nodes {
            if status == OPENED {
                count_opened += 1;
            } else {
                count_opened -= 1;
                if count_opened == 0 {
                    count_sections += 1
                }
            }
        }
        count_sections >= n
    }
    pub fn check_valid_cuts(_n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        Self::check_can_cut(3, rectangles.iter().map(|x| (x[0], x[2])))
            || Self::check_can_cut(3, rectangles.iter().map(|x| (x[1], x[3])))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 5;
        let rectangles = vec![
            vec![1, 0, 5, 2],
            vec![0, 2, 2, 4],
            vec![3, 2, 5, 3],
            vec![0, 4, 4, 5],
        ];
        assert_eq!(Solution::check_valid_cuts(n, rectangles), true);
    }

    #[test]
    fn test_example_2() {
        let n = 4;
        let rectangles = vec![
            vec![0, 0, 1, 1],
            vec![2, 0, 3, 4],
            vec![0, 2, 2, 3],
            vec![3, 0, 4, 3],
        ];
        assert_eq!(Solution::check_valid_cuts(n, rectangles), true);
    }

    #[test]
    fn test_example_3() {
        let n = 4;
        let rectangles = vec![
            vec![0, 2, 2, 4],
            vec![1, 0, 3, 2],
            vec![2, 2, 3, 4],
            vec![3, 0, 4, 2],
            vec![3, 2, 4, 4],
        ];
        assert_eq!(Solution::check_valid_cuts(n, rectangles), false);
    }
}
