use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included};
#[allow(dead_code)]
struct MyCalendarTwo {
    events: BTreeMap<i32, i32>,
    overlaps: BTreeMap<i32, i32>,
}

#[allow(dead_code)]
impl MyCalendarTwo {
    fn new() -> Self {
        Self {
            events: BTreeMap::new(),
            overlaps: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for rng in self.overlaps.range((Included(&0), Excluded(&end))) {
            if *rng.0 < end && *rng.1 > start {
                return false;
            }
        }
        for overlap in self
            .events
            .range((Included(&0), Excluded(&end)))
            .map(|(k, v)| (*k, *v))
        {
            if overlap.0 < end && overlap.1 > start {
                self.overlaps
                    .insert(overlap.0.max(start), overlap.1.min(end));
            }
        }
        if self.events.contains_key(&start) {
            let prev_end = self.events.get_mut(&start).unwrap();
            if *prev_end < end {
                *prev_end = end;
            }
        } else {
            self.events.insert(start, end);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_calendar_two() {
        let mut calendar = MyCalendarTwo::new();
        assert_eq!(calendar.book(10, 20), true);
        assert_eq!(calendar.book(50, 60), true);
        assert_eq!(calendar.book(10, 40), true);
        assert_eq!(calendar.book(5, 15), false);
        assert_eq!(calendar.book(5, 10), true);
        assert_eq!(calendar.book(25, 55), true);
    }

    #[test]
    fn test_my_calendar_two_custom() {
        let mut calendar = MyCalendarTwo::new();
        assert_eq!(calendar.book(24, 40), true);
        assert_eq!(calendar.book(43, 50), true);
        assert_eq!(calendar.book(27, 43), true);
        assert_eq!(calendar.book(5, 21), true);
        assert_eq!(calendar.book(30, 40), false);
        assert_eq!(calendar.book(14, 29), false);
        assert_eq!(calendar.book(3, 19), true);
        assert_eq!(calendar.book(3, 14), false);
        assert_eq!(calendar.book(25, 39), false);
        assert_eq!(calendar.book(6, 19), false);
    }
}
