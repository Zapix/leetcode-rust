struct Solution;

impl Solution {
    pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut count = 0;
        players.sort();
        trainers.sort();
        while i < players.len() && j < trainers.len() {
            if players[i] <= trainers[j] {
                count += 1;
                i += 1;
            }
            j += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let players = vec![4, 7, 9];
        let trainers = vec![8, 2, 5, 8];
        assert_eq!(Solution::match_players_and_trainers(players, trainers), 2);
    }

    #[test]
    fn test_example2() {
        let players = vec![1, 1, 1];
        let trainers = vec![10];
        assert_eq!(Solution::match_players_and_trainers(players, trainers), 1);
    }

    #[test]
    fn test_example3() {
        let players = vec![1, 1000000000];
        let trainers = vec![1000000000, 1];
        assert_eq!(Solution::match_players_and_trainers(players, trainers), 2);
    }
}
