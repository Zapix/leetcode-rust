use std::collections::HashMap;

#[allow(dead_code)]
struct MyCalendar {
    events: HashMap<i32, i32>,
}

#[allow(dead_code)]
impl MyCalendar {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }

    pub fn book(&mut self, start: i32, end: i32) -> bool {
        for (&s, &e) in self.events.iter() {
            if start < e && end > s {
                return false;
            }
        }
        self.events.insert(start, end);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        let mut calendar = MyCalendar::new();
        assert_eq!(calendar.book(10, 20), true);
        assert_eq!(calendar.book(15, 25), false);
        assert_eq!(calendar.book(20, 30), true);
    }
}
