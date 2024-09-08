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

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut result = vec![];
        if head.is_none() {
            return vec![];
        }
        let mut current = head.as_ref();
        let mut count = 0;

        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref();
        }
        let size = count / k;
        let mut rest = count % k;

        let mut current = head;

        for _ in 0..k {
            let mut part_size = size + if rest > 0 { 1 } else { 0 };
            let mut dummy = Box::new(ListNode::new(-1));
            let mut tail = &mut dummy;

            while part_size > 0 {
                tail.next = current.take();
                tail = tail.next.as_mut().unwrap();
                current = tail.next.take();
                part_size -= 1;
            }

            result.push(dummy.next.take());
            if (rest > 0) {
                rest -= 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        let k = 3;
        let result = Solution::split_list_to_parts(head, k);
        assert_eq!(3, result.len());
    }

    #[test]
    fn leetcode_2() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let k = 5;
        let result = Solution::split_list_to_parts(head, k);
        assert_eq!(5, result.len());
    }

    #[test]
    fn leetcode_3() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode {
                                val: 6,
                                next: Some(Box::new(ListNode {
                                    val: 7,
                                    next: Some(Box::new(ListNode {
                                        val: 8,
                                        next: Some(Box::new(ListNode {
                                            val: 9,
                                            next: Some(Box::new(ListNode {
                                                val: 10,
                                                next: None,
                                            })),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let k = 3;
        let result = Solution::split_list_to_parts(head, k);
        assert_eq!(3, result.len());
    }
}
