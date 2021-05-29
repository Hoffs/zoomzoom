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

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack: Vec<_> = Vec::new();
    if let Some(root_node) = &root {
        let x = Rc::clone(root_node);
        stack.push((x, 0));
    }

    // utilizing in order traversal by doing depth first search
    // go down furthest left, and then go back up every node to the right
    // idea:
    // have 2 vars, current_node, previous_node.
    // previous_node = the node we last visited,
    // current_ndoe = the node we are visiting,
    // previous_node always has to be less or equal to current_node
    //
    // to do that iterate all the way to the left down
    // then once theres no more left to go, pop off last value (the deepest left value)
    // => current_node
    // compare it to stored previous_node
    // assign previous_node = current_node
    // assign current_node = current_node.right
    // repeat
    // FOR RUST, instead of assignments (because cant assign to borrowed var) pust it to stack
    // instead. This introduces slight overhead, but should be fine?
    // ALTERNATIVE, is to go down the left/right recursively decreasing the range of values
    // [min, max], going left [min, root], going right [root, max]
    let mut previous_node: Option<Rc<RefCell<TreeNode>>> = None;

    while let Some((node, time)) = stack.pop() {
        // println!("#####################3\nprevious_node: {:#?}\ncurrent node: {:#?}\nstack:\n{:#?}", previous_node, node, stack);
        // go left with current node to the bottom

        let n = node.borrow();
        if time == 0 {
            if let Some(left) = &n.left {
                // left node exists, means we can go level down.
                // push starting node and then left,
                // pop will take out left node and try going down again.
                // when pushing same node increase the "time" so that when its
                // popped, to not traverse left again
                stack.push((Rc::clone(&node), time+1));
                stack.push((Rc::clone(left), 0));
                continue;
            }
        }

        // check if current node is less or equal to previous
        if let Some(prev) = previous_node {
            let prev_b = prev.borrow();
            if n.val <= prev_b.val {
                return false;
            }
        }

        // set previous to current
        previous_node = Some(Rc::clone(&node));

        // push right node if possible
        if let Some(right) = &n.right {
            stack.push((Rc::clone(right), 0));
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::{is_valid_bst, TreeNode};

    #[test]
    fn it_works() {
        let mut root = TreeNode::new(2);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        assert_eq!(is_valid_bst(Some(Rc::new(RefCell::new(root)))), true);
    }

    #[test]
    fn it_works_2() {
        let mut root = TreeNode::new(2);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

        assert_eq!(is_valid_bst(Some(Rc::new(RefCell::new(root)))), false);
    }

    #[test]
    fn it_works_false() {
        let mut root = TreeNode::new(5);
        let mut right_bst = TreeNode::new(4);
        right_bst.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        right_bst.right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.right = Some(Rc::new(RefCell::new(right_bst)));

        assert_eq!(is_valid_bst(Some(Rc::new(RefCell::new(root)))), false);
    }

    #[test]
    fn it_works_false_2() {
        let mut root = TreeNode::new(5);
        let mut right_bst = TreeNode::new(6);
        right_bst.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        right_bst.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.right = Some(Rc::new(RefCell::new(right_bst)));

        assert_eq!(is_valid_bst(Some(Rc::new(RefCell::new(root)))), false);
    }
}
