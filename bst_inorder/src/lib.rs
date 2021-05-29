use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut stack: Vec<_> = Vec::new();
    if let Some(root_node) = &root {
        let x = Rc::clone(root_node);
        stack.push((x, 0));
    }

    while let Some((node, time)) = stack.pop() {
        // go left with current node to the bottom

        let n = node.borrow();
        if time == 0 {
            if let Some(left) = &n.left {
                // left node exists, means we can go level down.
                // push starting node and then left,
                // pop will take out left node and try going down again.
                // when pushing same node increase the "time" so that when its
                // popped, to not traverse left again
                stack.push((Rc::clone(&node), time + 1));
                stack.push((Rc::clone(left), 0));
                continue;
            }
        }

        result.push(n.val);

        // push right node if possible
        if let Some(right) = &n.right {
            stack.push((Rc::clone(right), 0));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::{inorder_traversal, TreeNode};

    #[test]
    fn it_works() {
        let mut root = TreeNode::new(2);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        assert_eq!(
            inorder_traversal(Some(Rc::new(RefCell::new(root)))),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn it_works_false() {
        let mut root = TreeNode::new(5);
        let mut right_bst = TreeNode::new(7);
        right_bst.left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        right_bst.right = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.right = Some(Rc::new(RefCell::new(right_bst)));

        assert_eq!(
            inorder_traversal(Some(Rc::new(RefCell::new(root)))),
            vec![1, 5, 6, 7, 8]
        );
    }
}
