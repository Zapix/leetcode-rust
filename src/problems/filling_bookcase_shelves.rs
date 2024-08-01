use std::cmp::{max, min};

#[allow(dead_code)]
struct  Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut dp = Vec::<i32>::new();
        dp.push(0);
        dp.push(*(*books.get(0).unwrap()).get(1).unwrap());

        for i in 2..(books.len() + 1) {
            let book = books.get(i - 1).unwrap();
            let book_width = *book.get(0).unwrap();
            let book_height = *book.get(1).unwrap();
            dp.push(dp.get(dp.len() - 1).unwrap() + book_height);
            let mut remaining_width = shelf_width - book_width;

            let mut current_height = book_height;
            let mut j = i - 1;
            while j > 0 && remaining_width > 0 {
                let prev_book= books.get(j - 1).unwrap();
                let prev_width = *prev_book.get(0).unwrap();
                let prev_height = *prev_book.get(1).unwrap();
                remaining_width -= prev_width;
                if remaining_width < 0 {
                    break;
                }
                current_height = max(current_height, prev_height);
                let shelve_height =  dp.pop().unwrap();
                dp.push(min(shelve_height, current_height + dp.get(j - 1).unwrap()));
                j -= 1;
            }
        }

        dp.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::min_height_shelves(
                vec![
                    vec![1,1],
                    vec![2,3],
                    vec![2,3],
                    vec![1,1],
                    vec![1,1],
                    vec![1,1],
                    vec![1,2]
                ],
                4
            ),
            6
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::min_height_shelves(
                vec![
                    vec![1, 3],
                    vec![2, 4],
                    vec![3, 2],
                ],
                6
            ),
            4
        );
    }
}