#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    let (left, right) = (left as usize, right as usize);
    let mut list = vec![];
    let mut node = head;

    while let Some(n) = node.take() {
        node = n.next;
        list.push(n.val);
    }

    list[(left - 1)..right].reverse();
    list.reverse();
    let mut new = None;
    for n in list {
        let mut node = ListNode::new(n);
        node.next = new;
        new = Some(Box::new(node));
    }
    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_between() {
        assert_eq!(
            reverse_between(Some(Box::new(ListNode::new(5))), 1, 1),
            Some(Box::new(ListNode::new(5)))
        );
    }
}
