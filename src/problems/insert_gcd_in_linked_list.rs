use std::boxed::Box;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head.clone();
        let mut current = &mut head;
        while !current.is_none() {
            let a = current.as_ref().unwrap().val;
            if current.as_ref().unwrap().next.is_some() {
                let b = current.as_ref().unwrap().next.as_ref().unwrap().val;
                let div = Solution::gcd(a, b);
                let next = current.as_mut().unwrap().next.take();
                let mut item = Box::new(ListNode::new(div));
                item.next = next;
                current.as_mut().unwrap().next = Some(item);
                current = &mut current.as_mut().unwrap().next.as_mut().unwrap().next;
            } else {
                current = &mut current.as_mut().unwrap().next
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn array_to_linked_list(arr: &[i32]) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut current = &mut head;

        for &val in arr.iter().rev() {
            let new_node = Box::new(ListNode {
                val,
                next: current.take(),
            });
            *current = Some(new_node);
        }

        head
    }

    fn is_same_list(mut head1: &Option<Box<ListNode>>, mut head2: &Option<Box<ListNode>>) -> bool {
        while head1.is_some() && head2.is_some() {
            let node1 = head1.as_ref().unwrap();
            let node2 = head2.as_ref().unwrap();
            if node1.val != node2.val {
                return false;
            }
            head1 = &node1.next;
            head2 = &node2.next;
        }
        head1.is_none() && head2.is_none()
    }

    #[test]
    fn leetcode_1() {
        let head = array_to_linked_list(&[18, 6, 10, 3]);
        let result = Solution::insert_greatest_common_divisors(head);
        let expected = array_to_linked_list(&[18, 6, 6, 2, 10, 1, 3]);
        assert_eq!(is_same_list(&result, &expected), true);
    }

    #[test]
    fn leetcode_2() {
        let head = array_to_linked_list(&[2, 7, 4, 3, 5]);
        let result = Solution::insert_greatest_common_divisors(head);
        let expected = array_to_linked_list(&[2, 1, 7, 1, 4, 1, 3, 1, 5]);
        assert_eq!(is_same_list(&result, &expected), true);
    }

    #[test]
    fn leetcode_3() {
        let head = array_to_linked_list(&[7]);
        let result = Solution::insert_greatest_common_divisors(head);
        let expected = array_to_linked_list(&[7]);
        assert_eq!(is_same_list(&result, &expected), true);
    }
}
