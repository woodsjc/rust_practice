use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(self, val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut dq: VecDeque<(Option<Rc<RefCell<TreeNode>>>, usize)> = VecDeque::new();

    dq.push_back((root, 0));
    while let Some((node, depth)) = dq.pop_front() {
        if node.as_ref().is_none() {
            continue;
        }
        if result.len() < depth + 1 {
            result.push(vec![]);
        }
        let n = node.as_ref().unwrap().borrow();
        result[depth].push(n.val);
        dq.push_back((n.left.clone(), depth + 1));
        dq.push_back((n.right.clone(), depth + 1));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_order() {
        assert_eq!(
            level_order(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None
            })))),
            vec![vec![1]]
        );
    }
}
