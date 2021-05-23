// lc problem 146
// ideal solution is hashmap + linked list, where hash map is used to store values and linked list
// is used to maintain lru keys, changing next/prev when nodes are being touched/removed/added
// but linked lists are annoying in rust

// but with constraints being capacity <= 3000 and key <= 3000 we can use a vec with that size
// and mimic doubly linked list
// this yields:
// Runtime: 20 ms, faster than 100.00% of Rust online submissions for LRU Cache.
// Memory Usage: 11.5 MB, less than 93.75% of Rust online submissions for LRU Cache.
#[derive(Debug)]
struct Node {
    k: i32,
    v: i32,
    prev: i32,
    next: i32,
}

impl Node {
    fn reset(&mut self) {
        self.k = -1;
        self.v = -1;
        self.prev = -1;
        self.next = -1;
    }
}

impl Default for Node {
    fn default() -> Self {
        Node{k: -1, v: -1, prev: -1, next: -1}
    }
}

#[derive(Debug)]
struct LRUCache {
    cache: Vec<Node>,
    root: Node, // pseudo node, prev points to last, next points to first
    cap: usize,
    len: usize,
}


impl LRUCache {
    fn new(capacity: i32) -> Self {
        let mut c = LRUCache{cache: Vec::new(), cap: capacity as usize, len: 0, root: Node{ k: -1, v: -1, prev: -1, next: -1 }};
        c.cache.resize_with(3000, Default::default);
        c
    }

    fn get(&mut self, key: i32) -> i32 {
        let n = self.cache.get(key as usize).unwrap();
        if n.k != -1 {
            let next = n.next;
            let prev = n.prev;
            let v = n.v;
            // println!("{:#?}", n);
            self.remove(prev, next);
            self.add_to_front(key, v);
            v
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let n = self.cache.get_mut(key as usize).unwrap();
        if n.k != -1 {
            // remove node links
            let next = n.next;
            let prev = n.prev;
            self.remove(prev, next);
            self.len -= 1;
        } else {
            // if full, pop prev from root.
            if self.len == self.cap {
                // cap has to be at least 1, so if we have len == cap, prev/next should point
                // to some real node
                let last = self.cache.get_mut(self.root.prev as usize).unwrap();
                let last_p = last.prev;
                last.reset();

                if last_p == -1 {
                    // last previous was pointing to root
                    self.root.next = -1;
                    self.root.prev = -1;
                } else {
                    let prev = self.cache.get_mut(last_p as usize).unwrap();
                    prev.next = -1;
                    self.root.prev = last_p;
                }

                self.len -= 1;
            }
        }

        // add node to front
        self.add_to_front(key, value);
        self.len += 1;
    }

    fn remove(&mut self, prev: i32, next: i32) {
        if next == -1 {
            self.root.prev = prev;
        } else {
            let mut next = self.cache.get_mut(next as usize).unwrap();
            next.prev = prev;
        }

        if prev == -1 {
            self.root.next = next;
        } else {
            let mut prev = self.cache.get_mut(prev as usize).unwrap();
            prev.next = next;
        }
    }

    fn add_to_front(&mut self, key: i32, value: i32) {
        if self.root.next != -1 {
            // push first node back
            let mut root_next = self.cache.get_mut(self.root.next as usize).unwrap();
            root_next.prev = key;
        } else {
            self.root.prev = key;
        }

        let new_node = self.cache.get_mut(key as usize).unwrap();
        new_node.k = key;
        new_node.v = value;
        new_node.prev = -1;
        new_node.next = self.root.next;
        self.root.next = key;
    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    #[test]
    fn pushes_to_front() {
        let mut c = LRUCache::new(3);

        c.put(15, 23);
        c.put(17, 24);
        c.put(12, 25);

        println!("{:#?}", c.root);
        assert_eq!(12, c.root.next);
        assert_eq!(15, c.root.prev);
    }

    #[test]
    fn pushes_to_front_with_get() {
        let mut c = LRUCache::new(3);

        c.put(15, 23);
        c.put(17, 24);
        c.put(12, 25);

        println!("before get {:#?}", c.root);
        c.get(15);

        println!("after get {:#?}", c.root);
        assert_eq!(15, c.root.next);
        assert_eq!(17, c.root.prev);
    }

    #[test]
    fn something() {
        let mut c = LRUCache::new(2);

        c.put(1, 1);
        c.put(2, 2);
        c.get(1);
        c.put(3, 3);
        assert_eq!(-1, c.get(2));
        c.put(4, 4);
        assert_eq!(-1, c.get(1));
        assert_eq!(3, c.get(3));
        assert_eq!(4, c.get(4));
    }

    #[test]
    fn something_2() {
        let mut c = LRUCache::new(2);

        c.put(2, 1);
        c.put(1, 1);
        c.put(2, 3);
        c.put(4, 1);

        assert_eq!(-1, c.get(1));
        assert_eq!(3, c.get(2));
    }
}
