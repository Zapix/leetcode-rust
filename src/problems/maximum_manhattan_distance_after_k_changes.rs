struct Solution;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    NE,
    NW,
    SE,
    SW,
}

impl Solution {
    pub fn max_by_direction(s: &str, k: i32, direction: Direction) -> i32 { 
        let mut count = 0;
        let mut max_distance = 0;
        let mut current_distance = 0;

        for c in s.chars() {
            match c {
                'N' if direction == Direction::NE || direction == Direction::NW => current_distance += 1,
                'S' if direction == Direction::SE || direction == Direction::SW => current_distance += 1,
                'E' if direction == Direction::NE || direction == Direction::SE => current_distance += 1,
                'W' if direction == Direction::NW || direction == Direction::SW => current_distance += 1,
                _ => {
                    if count < k {
                        count += 1;
                        current_distance += 1;
                    } else {
                        current_distance -= 1;
                    }
                }
            }
            max_distance = max_distance.max(current_distance);
        }

        max_distance
    }
    
    pub fn max_distance(s: String, k: i32) -> i32 {
        let mut max_distance = 0;

        // Check for each direction
        max_distance = max_distance.max(Self::max_by_direction(&s, k, Direction::NE));
        max_distance = max_distance.max(Self::max_by_direction(&s, k, Direction::NW));
        max_distance = max_distance.max(Self::max_by_direction(&s, k, Direction::SE));
        max_distance = max_distance.max(Self::max_by_direction(&s, k, Direction::SW));

        max_distance
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "NWSE".to_string();
        let k = 1;
        assert_eq!(Solution::max_distance(s, k), 3);
    }

    #[test]
    fn test_example_2() {
        let s = "NSWWEW".to_string();
        let k = 3;
        assert_eq!(Solution::max_distance(s, k), 6);
    }
}
