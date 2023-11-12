use std::collections::{HashMap as Map, VecDeque, HashSet as Set};
#[allow(dead_code)]
struct Solution;

impl Solution {
    fn has_common_stop(routes: &Vec<Vec<i32>>, route1: usize, route2: usize) -> bool {
        let mut i = 0;
        let mut j = 0;
        while i < routes[route1].len() && j < routes[route2].len() {
            if routes[route1][i] == routes[route2][j] {
                return true;
            } else if routes[route1][i] < routes[route2][j] {
                i += 1;
            } else {
                j += 1;
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        let mut routes = routes.to_owned();
        if source == target {
            return 0;
        }

        for route in routes.iter_mut() {
            route.sort();
        }

        let mut route_2_routes = vec![vec![false; routes.len()]; routes.len()];

        for i in 0..routes.len() {
            for j in (i + 1)..routes.len() {
                if Solution::has_common_stop(&routes, i, j) {
                    route_2_routes[i][j] = true;
                    route_2_routes[j][i] = true;
                }
            }
        }

        let mut queue: VecDeque<(usize, i32)> = VecDeque::new();
        let mut visited: Set<usize> = Set::new();

        for i in 0..routes.len() {
            if routes[i].contains(&source) {
                queue.push_back((i, 1));
                visited.insert(i);
            }
        }

        while !queue.is_empty() {
            let (route, cnt) = queue.pop_front().unwrap();
            if routes[route].contains(&target) {
                return cnt;
            }
            for i in 0..routes.len() {
                if route_2_routes[route][i] && !visited.contains(&i) {
                    queue.push_back((i, cnt + 1));
                    visited.insert(i);
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_1() {
        assert_eq!(
            Solution::num_buses_to_destination(
                vec![
                    vec![1, 2, 7],
                    vec![3, 6, 7]
                ],
                1,
                6
            ),
            2
        );
    }

    #[test]
    fn simple_2() {
        assert_eq!(
            Solution::num_buses_to_destination(
                vec![
                    vec![7, 12],
                    vec![4, 5, 15],
                    vec![6],
                    vec![15, 19],
                    vec![9, 12, 13]
                ],
                15,
                12
            ),
            -1
        );
    }

    #[test]
    fn simple_same_source_and_target() {
        assert_eq!(
            Solution::num_buses_to_destination(
                vec![
                    vec![1, 2, 7],
                    vec![3, 6, 7]
                ],
                1,
                1
            ),
            0
        );
    }

    #[test]
    fn simple_two_buses_should_be_changed() {
        assert_eq!(
            Solution::num_buses_to_destination(
                vec![
                    vec![1, 2, 7],
                    vec![3, 6, 7],
                    vec![4, 5, 6]
                ],
                1,
                6
            ),
            2
        );
    }

    #[test]
    fn simple_4() {
        assert_eq!(
            Solution::num_buses_to_destination(
                vec![
                    vec![1],
                    vec![15,16,18],
                    vec![10],
                    vec![3,4,12,14]
                ],
                3,
               15
            ),
            -1
        );
    }
}
