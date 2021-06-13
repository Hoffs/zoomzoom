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
use std::cell::RefCell;
use std::rc::Rc;
struct Codec {}

// Simple DFS-style serialization.
// Encode visiting all nodes going left, if left is null, write nil, and go down right, if right is
// null, write nil and go back up.
// So basically its just DFS.
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        use std::fmt::Write;
        let mut s = String::new();
        let mut q = Vec::new();
        if let Some(r) = root {
            q.push((Rc::clone(&r), 0));
        }

        while let Some((node, time)) = q.pop() {
            let n = node.borrow();
            if time == 0 {
                write!(&mut s, "{},", n.val).expect("failed to write");
                if let Some(l_node) = &n.left {
                    q.push((Rc::clone(&node), time + 1));
                    q.push((Rc::clone(l_node), 0));
                    continue;
                } else {
                    write!(&mut s, "nil,").expect("failed to write");
                }
            }

            if let Some(r_node) = &n.right {
                q.push((Rc::clone(r_node), 0));
            } else {
                write!(&mut s, "nil,").expect("failed to write");
            }
        }
        s
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.len() == 0 {
            return None;
        }

        let mut split = data[..data.len() - 1].split(",");
        let mut q = Vec::new();
        let root = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut current = Rc::clone(&root);
        let mut check_left = true;
        if let Some(first) = split.next() {
            if first != "nil" {
                let mut r_b = root.borrow_mut();
                r_b.val = first.parse::<i32>().unwrap();
            }
        }

        q.push(Rc::clone(&root));

        while let Some(n) = split.next() {
            if n != "nil" {
                let val = n.parse::<i32>().unwrap();
                let node = Rc::new(RefCell::new(TreeNode::new(val)));
                {
                    let mut x_b = current.borrow_mut();
                    match check_left {
                        true => {
                            x_b.left = Some(Rc::clone(&node));
                            q.push(Rc::clone(&current));
                        }
                        false => x_b.right = Some(Rc::clone(&node)),
                    }
                }
                check_left = true;
                current = node;
            } else {
                if !check_left {
                    current = q.pop().unwrap();
                }
                check_left = false;
            }
        }

        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::{Codec, TreeNode};

    #[test]
    fn it_works() {
        let mut t = TreeNode::new(5);
        t.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let c = Codec::new();
        let t_r = Rc::new(RefCell::new(t));
        let serialized = c.serialize(Some(Rc::clone(&t_r)));
        println!("{}", serialized);
        let deserialized = c.deserialize(serialized).unwrap();
        let d_b = deserialized.borrow();

        assert_eq!(*t_r.borrow(), *d_b);
    }

    #[test]
    fn it_works_2() {
        let mut right = TreeNode::new(3);
        right.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        right.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let mut t = TreeNode::new(1);
        t.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        t.right = Some(Rc::new(RefCell::new(right)));
        let c = Codec::new();
        let t_r = Rc::new(RefCell::new(t));
        let serialized = c.serialize(Some(Rc::clone(&t_r)));
        println!("{}", serialized);
        let deserialized = c.deserialize(serialized).unwrap();
        let d_b = deserialized.borrow();

        assert_eq!(*t_r.borrow(), *d_b);
    }

    #[test]
    fn it_works_3() {
        let mut right_left = TreeNode::new(4);
        right_left.left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        right_left.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let mut right = TreeNode::new(3);
        right.left = Some(Rc::new(RefCell::new(right_left)));
        right.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let mut t = TreeNode::new(1);
        t.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        t.right = Some(Rc::new(RefCell::new(right)));
        let c = Codec::new();
        let t_r = Rc::new(RefCell::new(t));
        let serialized = c.serialize(Some(Rc::clone(&t_r)));
        println!("{}", serialized);
        let deserialized = c.deserialize(serialized).unwrap();
        let d_b = deserialized.borrow();

        assert_eq!(*t_r.borrow(), *d_b);
    }
}
