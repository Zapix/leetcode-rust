use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Node {
    value: String,
    count: usize,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

#[allow(dead_code)]
struct AllOne {
    words: HashMap<String, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

#[allow(dead_code)]
impl AllOne {
    pub fn new() -> Self {
        let head = Rc::new(RefCell::new(Node {
            value: "".to_string(),
            count: 0,
            prev: None,
            next: None,
        }));

        let tail = Rc::new(RefCell::new(Node {
            value: "".to_string(),
            count: usize::MAX,
            prev: None,
            next: None,
        }));
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());

        Self {
            words: HashMap::new(),
            head,
            tail,
        }
    }
    pub fn inc(&mut self, key: String) {
        if self.words.contains_key(&key) {
            let node = self.words.get(&key).unwrap();
            node.borrow_mut().count += 1;
            while node.borrow().next.clone().unwrap().borrow().count < node.borrow().count {
                let prev = node.borrow().prev.clone().unwrap();
                let next = node.borrow().next.clone().unwrap();
                let next_next = next.borrow().next.clone().unwrap();
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());

                node.borrow_mut().prev = Some(next.clone());
                node.borrow_mut().next = Some(next_next.clone());

                next.borrow_mut().next = Some(node.clone());
                next_next.borrow_mut().prev = Some(node.clone());
            }
        } else {
            let head_next = self.head.borrow().next.clone().unwrap();
            let new_node = Rc::new(RefCell::new(Node {
                value: key.clone(),
                count: 1,
                prev: Some(self.head.clone()),
                next: Some(head_next.clone()),
            }));
            self.head.borrow_mut().next = Some(new_node.clone());
            head_next.borrow_mut().prev = Some(new_node.clone());
            self.words.insert(key, new_node);
        }
    }
    pub fn dec(&mut self, key: String) {
        if !self.words.contains_key(&key) {
            return;
        }
        let node = self.words.get(&key).unwrap();

        if node.borrow().count == 1 {
            let prev = node.borrow().prev.clone().unwrap();
            let next = node.borrow().next.clone().unwrap();
            prev.borrow_mut().next = Some(next.clone());
            next.borrow_mut().prev = Some(prev.clone());
            self.words.remove(&key);
            return;
        }

        node.borrow_mut().count -= 1;
        while node.borrow().count < node.borrow().prev.clone().unwrap().borrow().count {
            let prev = node.borrow().prev.clone().unwrap();
            let prev_prev = prev.borrow().prev.clone().unwrap();
            let next = node.borrow().next.clone().unwrap();

            prev_prev.borrow_mut().next = Some(node.clone());
            node.borrow_mut().prev = Some(prev_prev.clone());
            node.borrow_mut().next = Some(prev.clone());

            prev.borrow_mut().prev = Some(node.clone());
            prev.borrow_mut().next = Some(next.clone());
            next.borrow_mut().prev = Some(prev.clone());
        }
    }
    pub fn get_max_key(&self) -> String {
        self.tail
            .borrow()
            .prev
            .clone()
            .unwrap()
            .borrow()
            .value
            .to_string()
    }
    pub fn get_min_key(&self) -> String {
        self.head
            .borrow()
            .next
            .clone()
            .unwrap()
            .borrow()
            .value
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut all_one = AllOne::new();
        assert_eq!(all_one.get_max_key(), "".to_string());
        assert_eq!(all_one.get_min_key(), "".to_string());
    }

    #[test]
    fn test_add_one() {
        let mut all_one = AllOne::new();
        all_one.inc("hello".to_string());
        assert_eq!(all_one.get_max_key(), "hello".to_string());
        assert_eq!(all_one.get_min_key(), "hello".to_string());
    }

    #[test]
    fn test_all_one() {
        let mut all_one = AllOne::new();
        all_one.inc("hello".to_string());
        all_one.inc("hello".to_string());
        assert_eq!(all_one.get_max_key(), "hello".to_string());
        assert_eq!(all_one.get_min_key(), "hello".to_string());
        all_one.inc("leet".to_string());
        assert_eq!(all_one.get_max_key(), "hello".to_string());
        assert_eq!(all_one.get_min_key(), "leet".to_string());
    }

    #[test]
    fn test_all_one_2() {
        let mut all_one = AllOne::new();
        all_one.inc("hello".to_string());
        all_one.inc("hello".to_string());
        all_one.inc("leet".to_string());
        all_one.inc("leet".to_string());
        all_one.inc("leet".to_string());
        assert_eq!(all_one.get_max_key(), "leet".to_string());
        assert_eq!(all_one.get_min_key(), "hello".to_string());
        all_one.dec("leet".to_string());
        all_one.dec("leet".to_string());
        assert_eq!(all_one.get_max_key(), "hello".to_string());
        assert_eq!(all_one.get_min_key(), "leet".to_string());
    }

    #[test]
    fn test_all_one_3() {
        let mut all_one = AllOne::new();
        all_one.inc("hello".to_string());
        all_one.inc("hello".to_string());
        all_one.inc("leet".to_string());
        all_one.inc("leet".to_string());
        all_one.inc("leet".to_string());
        all_one.dec("leet".to_string());
        all_one.dec("leet".to_string());
        all_one.dec("leet".to_string());
        assert_eq!(all_one.get_max_key(), "hello".to_string());
        assert_eq!(all_one.get_min_key(), "hello".to_string());
    }

    #[test]
    fn test_all_one_jumping() {
        let mut all_one = AllOne::new();
        all_one.inc("a".to_string());
        all_one.inc("a".to_string());
        all_one.inc("b".to_string());
        all_one.inc("b".to_string());
        all_one.inc("c".to_string());
        all_one.inc("c".to_string());
        all_one.inc("d".to_string());
        all_one.inc("d".to_string());
        all_one.inc("d".to_string());
        assert_eq!(all_one.get_max_key(), "d".to_string());
        all_one.dec("d".to_string());
        all_one.dec("d".to_string());
        assert_eq!(all_one.get_min_key(), "d".to_string());
    }
}
