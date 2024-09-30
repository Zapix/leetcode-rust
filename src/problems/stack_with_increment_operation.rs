#[allow(dead_code)]
struct CustomStack {
    capacity: i32,
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    fn new(maxSize: i32) -> Self {
        Self {
            capacity: maxSize,
            stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() < self.capacity as usize {
            self.stack.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        let size = self.stack.len() as i32;
        for item in self.stack.iter_mut().take(k.min(size) as usize) {
            *item += val;
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = CustomStack::new(3);
        obj.push(1);
        obj.push(2);
        obj.push(3);
        obj.push(4);
        assert_eq!(obj.pop(), 3);
        obj.push(5);
        obj.push(6);
        obj.increment(2, 100);
        assert_eq!(obj.pop(), 5);
        assert_eq!(obj.pop(), 102);
        assert_eq!(obj.pop(), 101);
    }

    #[test]
    fn test_2() {
        let mut obj = CustomStack::new(3);
        obj.push(1);
        obj.push(2);
        obj.increment(2, 100);
        obj.push(3);
        obj.push(4);
        obj.increment(10, 100);
        assert_eq!(obj.pop(), 103);
        assert_eq!(obj.pop(), 202);
        assert_eq!(obj.pop(), 201);
        assert_eq!(obj.pop(), -1);
    }
}
