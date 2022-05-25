#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carry: i32 = 0;
    let mut c: i32;
    let mut head = ListNode::new(0);
    let mut result = &mut head;
    let (mut i, mut j);
    let (mut l1, mut l2) = (l1, l2);

    while l1 != None || l2 != None || carry > 0 {
        match l1 {
            Some(x) => {
                i = x.val;
                l1 = x.next;
            }
            None => i = 0,
        }
        match l2 {
            Some(x) => {
                j = x.val;
                l2 = x.next;
            }
            None => j = 0,
        }
        print!("i.val:{}, j.val:{}, carry:{}\n", i, j, carry);
        c = i + j + carry;

        if c >= 10 {
            carry = c / 9;
            c = c % 10
        } else {
            carry = 0;
        }

        result.next = Some(Box::new(ListNode::new(c)));
        result = result.next.as_mut().unwrap();
    }

    head.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let n1 = Some(Box::new(ListNode { val: 9, next: None }));
        let n2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode {
                                    val: 9,
                                    next: Some(Box::new(ListNode {
                                        val: 9,
                                        next: Some(Box::new(ListNode {
                                            val: 9,
                                            next: Some(Box::new(ListNode { val: 9, next: None })),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let ans = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode {
                                        val: 0,
                                        next: Some(Box::new(ListNode {
                                            val: 0,
                                            next: Some(Box::new(ListNode {
                                                val: 0,
                                                next: Some(Box::new(ListNode {
                                                    val: 1,
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
            })),
        }));

        // i32 overflow
        assert_eq!(add_two_numbers(n1, n2), ans);
    }
}
