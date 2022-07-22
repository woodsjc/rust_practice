#[derive(Debug, Clone, Eq, PartialEq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut before = vec![];
    let mut after = vec![];
    let mut node = head;

    while let Some(n) = node.take() {
        if n.val < x {
            before.push(n.val);
        } else {
            after.push(n.val);
        }
        node = n.next;
    }

    let mut new = None;
    while let Some(n) = after.pop() {
        new = Some(Box::new(ListNode{val: n, next: new}));
    }
    while let Some(n) = before.pop() {
        new = Some(Box::new(ListNode{val: n, next: new}));
    }

    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        assert_eq!(
            partition(
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 1, next: None }))
                })),
                2
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            }))
        );
    }
}
