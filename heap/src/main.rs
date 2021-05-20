mod data {
    use std::fmt;

    pub trait Prioritable {
        fn priority(&self) -> i64;
    }

    #[derive(Debug)]
    pub enum HeapError {
        Full,
        Unknown,
    }

    #[derive(Debug)]
    pub struct MaxHeap<T: Prioritable + Eq> {
        data: Vec<T>,
    }

    // given a node at index i
    // - children at 2i + 1 and 2i + 2,
    // - parent at floor((i-1)/2)
    impl<T: Prioritable + Eq + Clone + fmt::Debug> MaxHeap<T> {
        pub fn new(size: usize) -> MaxHeap<T> {
            return MaxHeap {
                data: Vec::with_capacity(size),
            };
        }

        pub fn from_vec(vec: Vec<T>) -> MaxHeap<T> {
            let mut h = MaxHeap { data: vec };

            for i in (0..h.data.len()).rev() {
                h.sift_up(i);
            }

            return h;
        }

        // Insert at last position, then rebalance
        pub fn add<'a>(&'a mut self, element: T) -> Result<&'a T, HeapError> {
            if self.data.len() == self.data.capacity() {
                return Result::Err(HeapError::Full);
            }

            let index = self.data.len();
            self.data.insert(index, element);

            self.sift_up(index);

            self.ensure_valid();
            if let Some(el) = self.data.get(index) {
                return Result::Ok(el);
            } else {
                return Result::Err(HeapError::Unknown);
            }
        }

        fn sift_up(&mut self, pos: usize) {
            if let Some(el) = self.data.get(pos) {
                let priority = el.priority();
                let mut index = pos;
                loop {
                    if index == 0 {
                        break;
                    }

                    let parent = ((index - 1) as f64 / 2.0).floor() as usize;

                    if let Some(parent_el) = self.data.get(parent) {
                        let parent_priority = parent_el.priority();
                        if parent_priority >= priority {
                            break;
                        }

                        self.data.swap(parent, index);
                        index = parent;
                    }
                }
            }
        }

        pub fn pop(&mut self) -> Option<T> {
            if !self.data.is_empty() {
                let len = self.data.len() - 1;
                self.data.swap(0, len); // swap root with last
                let ret = self.data.remove(len); // remove last element

                // sift root node
                let mut index = 0;
                loop {
                    if self.data.is_empty() {
                        break;
                    }
                    if let Some(el) = self.data.get(index) {
                        let elpr = el.priority();
                        let child1idx = (index * 2) + 1;
                        let child1 = self.data.get(child1idx);
                        let child2idx = (index * 2) + 2;
                        let child2 = self.data.get(child2idx);

                        match (child1, child2) {
                            (Some(child1el), Some(child2el)) => {
                                let ch1pr = child1el.priority();
                                let ch2pr = child2el.priority();
                                if ch1pr <= elpr && ch2pr <= elpr {
                                    break;
                                }

                                if ch1pr >= ch2pr {
                                    self.data.swap(child1idx, index);
                                    index = child1idx;
                                } else {
                                    self.data.swap(child2idx, index);
                                    index = child2idx;
                                }
                            }
                            (None, Some(childel)) => {
                                let chpr = childel.priority();
                                if chpr <= elpr {
                                    break;
                                } else {
                                    self.data.swap(child2idx, index);
                                    index = child2idx;
                                }
                            }
                            (Some(childel), None) => {
                                let chpr = childel.priority();
                                if chpr <= elpr {
                                    break;
                                } else {
                                    self.data.swap(child1idx, index);
                                    index = child1idx;
                                }
                            }
                            (None, None) => {
                                break;
                            }
                        }
                    }
                }

                self.ensure_valid();
                return Option::Some(ret);
            }

            return Option::None;
        }

        fn ensure_valid(&self) {
            // verify by checking that every node satisfies requirement that parent is more than
            // the node
            for i in 1..self.data.len() {
                if let Some(node) = self.data.get(i) {
                    let parent = ((i - 1) as f64 / 2.0).floor() as usize;
                    if let Some(parentel) = self.data.get(parent) {
                        if parentel.priority() < node.priority() {
                            panic!(
                                "heap invalid at node {}/{} with parent at {}/{}, heap {:#?}",
                                i,
                                node.priority(),
                                parent,
                                parentel.priority(),
                                self.data
                            );
                        }
                    }
                }
            }
        }
    }
}

use data::{HeapError, MaxHeap, Prioritable};

impl Prioritable for i64 {
    fn priority(&self) -> i64 {
        return *self;
    }
}

fn main() -> Result<(), HeapError> {
    let mut heap: MaxHeap<i64> = MaxHeap::new(100);
    heap.add(100)?;
    heap.add(19)?;
    heap.add(25)?;
    heap.add(1)?;
    heap.add(36)?;
    heap.add(17)?;
    heap.add(2)?;
    heap.add(7)?;
    heap.add(3)?;
    heap.add(125)?;

    println!("{:#?}", heap);

    heap.pop();
    println!("{:#?}", heap);
    heap.pop();
    println!("{:#?}", heap);
    heap.pop();
    println!("{:#?}", heap);
    heap.pop();
    heap.pop();
    heap.pop();
    heap.pop();
    heap.pop();
    heap.pop();
    println!("{:#?}", heap);
    heap.pop();
    println!("{:#?}", heap);


    let mut heap: MaxHeap<i64> = MaxHeap::from_vec(vec![19, 2, 51, 11, 9, 1, 7, 15]);
    println!("{:#?}", heap);

    heap.pop();
    heap.pop();
    heap.pop();
    heap.pop();
    heap.pop();
    println!("{:#?}", heap);
    heap.pop();
    heap.pop();
    println!("{:#?}", heap);
    heap.pop();

    return Result::Ok(());
}
