use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

struct MyCircularDeque {
    size: i32,
    capacity: i32,
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        let head = Rc::new(RefCell::new(Node {
            value: 0,
            next: None,
            prev: None,
        }));
        let tail = Rc::new(RefCell::new(Node {
            value: 0,
            next: None,
            prev: None,
        }));
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());

        Self {
            size: 0,
            capacity: k,
            head,
            tail,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.size == self.capacity {
            return false;
        }

        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }));
        let next = self.head.borrow().next.clone().unwrap();
        self.head.borrow_mut().next = Some(new_node.clone());
        new_node.borrow_mut().prev = Some(self.head.clone());
        new_node.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(new_node.clone());
        self.size += 1;

        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.size == self.capacity {
            return false;
        }

        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }));
        let prev = self.tail.borrow().prev.clone().unwrap();
        self.tail.borrow_mut().prev = Some(new_node.clone());
        new_node.borrow_mut().prev = Some(prev.clone());
        new_node.borrow_mut().next = Some(self.tail.clone());
        prev.borrow_mut().next = Some(new_node.clone());
        self.size += 1;

        true
    }

    fn delete_front(&mut self) -> bool {
        if self.size == 0 {
            return false;
        }

        let next = self.head.borrow().next.clone().unwrap();
        let next_next = next.borrow().next.clone().unwrap();
        self.head.borrow_mut().next = Some(next_next.clone());
        next_next.borrow_mut().prev = Some(self.head.clone());
        self.size -= 1;

        true
    }

    fn delete_last(&mut self) -> bool {
        if self.size == 0 {
            return false;
        }

        let prev = self.tail.borrow().prev.clone().unwrap();
        let prev_prev = prev.borrow().prev.clone().unwrap();
        self.tail.borrow_mut().prev = Some(prev_prev.clone());
        prev_prev.borrow_mut().next = Some(self.tail.clone());
        self.size -= 1;

        true
    }

    fn get_front(&self) -> i32 {
        if self.size == 0 {
            return -1;
        }

        self.head.borrow().next.clone().unwrap().borrow().value
    }

    fn get_rear(&self) -> i32 {
        if self.size == 0 {
            return -1;
        }

        self.tail.borrow().prev.clone().unwrap().borrow().value
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let mut obj = MyCircularDeque::new(3);
        assert_eq!(obj.insert_last(1), true);
        assert_eq!(obj.insert_last(2), true);
        assert_eq!(obj.insert_front(3), true);
        assert_eq!(obj.insert_front(4), false);
        assert_eq!(obj.get_rear(), 2);
        assert_eq!(obj.is_full(), true);
        assert_eq!(obj.delete_last(), true);
        assert_eq!(obj.insert_front(4), true);
        assert_eq!(obj.get_front(), 4);
    }

    #[test]
    fn leetcode_2() {
        let mut obj = MyCircularDeque::new(3);
        assert_eq!(obj.insert_front(9), true);
        assert_eq!(obj.get_rear(), 9);
        assert_eq!(obj.insert_front(9), true);
        assert_eq!(obj.get_rear(), 9);
        assert_eq!(obj.insert_last(5), true);
        assert_eq!(obj.get_front(), 9);
        assert_eq!(obj.get_rear(), 5);
        assert_eq!(obj.get_front(), 9);
        assert_eq!(obj.insert_last(8), false);
        assert_eq!(obj.delete_last(), true);
        assert_eq!(obj.get_front(), 9);
    }
}
