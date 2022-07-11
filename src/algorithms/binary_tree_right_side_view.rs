use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }
    let mut stack = vec![(0, root)];
    let mut tree_vec: Vec<Vec<i32>> = vec![vec![]];

    while stack.len() > 0 {
        let (depth, node) = stack.remove(0);
        let node = node.as_ref().unwrap().borrow();
        let left = &node.left;
        let right = &node.right;

        if tree_vec.len() <= depth {
            tree_vec.push(vec![]);
        }
        tree_vec[depth].push(node.val);

        if left.is_some() {
            stack.push((depth + 1, left.as_ref().map(Rc::clone)));
        }
        if right.is_some() {
            stack.push((depth + 1, right.as_ref().map(Rc::clone)));
        }
        println!(
            "depth:{}, node.val:{}, stack.len():{:?}",
            depth,
            node.val,
            stack.len()
        );
    }
    tree_vec.iter().map(|a| *a.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_side_view() {
        //let input = vec![1,2,3,null,5,null,4];
        assert_eq!(
            right_side_view(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
            })))),
            vec![1, 3, 4]
        );
    }
}
