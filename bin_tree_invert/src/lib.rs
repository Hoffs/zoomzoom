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

fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(r) = root {
        invert_tree_rec(Rc::clone(&r));
        return Some(r)
    }

    return None
}

fn invert_tree_rec(node: Rc<RefCell<TreeNode>>) {
    let mut n = node.borrow_mut();
    if let Some(l) = &n.left {
        invert_tree_rec(Rc::clone(l));
    }

    if let Some(r) = &n.right {
        invert_tree_rec(Rc::clone(r));
    }

    let mut temp = n.left.take();
    n.left = n.right.take();
    n.right = temp.take();
}

#[cfg(test)]
mod tests {
use std::cell::RefCell;
use std::rc::Rc;
    use super::{invert_tree, TreeNode};
    #[test]
    fn it_works() {
        let mut t1 = TreeNode::new(2);
        t1.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        t1.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let mut t2 = TreeNode::new(2);
        t2.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        t2.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        assert_eq!(invert_tree(Some(Rc::new(RefCell::new(t1)))), Some(Rc::new(RefCell::new(t2))));
    }
}
