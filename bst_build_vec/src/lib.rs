use std::{cell::RefCell, rc::Rc};

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

fn sorted_to_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    // basically binary search but creating nodes
    if nums.len() == 0 {
        return None
    }

    let mid = nums.len() / 2;

    let left = &nums[..mid];
    let right = &nums[mid+1..];

    let mut node = TreeNode::new(nums[mid]);
    node.left = sorted_to_bst(left);
    node.right = sorted_to_bst(right);
    Some(Rc::new(RefCell::new(node)))
}

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    sorted_to_bst(&nums[..])
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::{sorted_array_to_bst, TreeNode};

    #[test]
    fn it_works() {
        let mut root = TreeNode::new(2);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        assert_eq!(
            sorted_array_to_bst(vec![1, 2, 3]),
            Some(Rc::new(RefCell::new(root)))
        );
    }

    #[test]
    fn it_works_false() {
        let mut root = TreeNode::new(6);
        let mut left_bst = TreeNode::new(5);
        left_bst.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let mut right_bst = TreeNode::new(8);
        right_bst.left = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.left = Some(Rc::new(RefCell::new(left_bst)));
        root.right = Some(Rc::new(RefCell::new(right_bst)));

        assert_eq!(
            sorted_array_to_bst(vec![1, 5, 6, 7, 8]),
            Some(Rc::new(RefCell::new(root)))
        );
    }
}
