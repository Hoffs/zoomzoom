pub trait Hash {
    fn hash(&self) -> usize;
}

#[derive(Default)]
#[derive(Debug)]
struct Bucket<K: Eq, T> {
    values: Vec<(K, T)>,
}

#[derive(Debug)]
struct SimpleHt<K: Hash + Eq, T> {
    buckets: Vec<Bucket<K, T>>,
    size: usize,
}

impl<K: Eq, T> Bucket<K, T> {
    pub fn get(&self, key: &K) -> Option<&T> {
        for (k, v) in self.values.iter() {
            if key == k {
                return Option::Some(v)
            }
        }

        Option::None
    }

    pub fn add(&mut self, key: K, value: T) -> bool {
        for (k, v) in self.values.iter() {
            if &key == k {
                return false
            }
        }

        self.values.push((key, value));
        true
    }

    pub fn remove(&mut self, key: &K) -> Option<T> {
        for (i,(k, _)) in self.values.iter().enumerate() {
            if key == k {
                let ret = self.values.remove(i);
                return Option::Some(ret.1)
            }
        }

        Option::None
    }
}

impl<K: Hash + Eq + Default, T: Default> SimpleHt<K, T> {
    pub fn new(size: usize) -> Self {
        let mut ht = SimpleHt { buckets: Vec::new(), size };
        ht.buckets.resize_with(size, Default::default);
        ht
    }

    pub fn add(&mut self, key: K, value: T) -> bool {
        let bucket = key.hash() % self.size;
        match self.buckets.get_mut(bucket) {
            Some(b) => {
                b.add(key, value)
            },
            None => {
                // Vector should be filled already
                false
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&T> {
        let bucket = key.hash() % self.size;
        match self.buckets.get(bucket) {
            Some(b) => {
                b.get(key)
            },
            None => {
                Option::None
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<T> {
        let bucket = key.hash() % self.size;
        match self.buckets.get_mut(bucket) {
            Some(b) => {
                b.remove(key)
            },
            None => {
                Option::None
            }
        }
    }
}

impl Hash for String {
    fn hash(&self) -> usize {
        // I mean, who says this isn't a hash function
        // Good for collisions
        return self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleHt;

    #[test]
    fn add_entry() {
        let mut ht: SimpleHt<String, i64> = SimpleHt::new(10);

        ht.add(String::from("abc"), 123);
        ht.add(String::from("abcd"), 124);

        assert_eq!(10, ht.buckets.len());

        let v = ht.get(&String::from("abc")).unwrap();
        assert_eq!(&123, v);

        let v = ht.get(&String::from("abcd")).unwrap();
        assert_eq!(&124, v);

        let v = ht.get(&String::from("abcdf"));
        assert_eq!(Option::None, v);
    }

    #[test]
    fn add_remove() {
        let mut ht: SimpleHt<String, i64> = SimpleHt::new(10);

        ht.add(String::from("abc"), 123);

        let v = ht.get(&String::from("abc")).unwrap();
        assert_eq!(&123, v);

        let v = ht.remove(&String::from("abc")).unwrap();
        assert_eq!(123, v);

        let v = ht.get(&String::from("abc"));
        assert_eq!(Option::None, v);
    }

    #[test]
    fn remove_empty() {
        let mut ht: SimpleHt<String, i64> = SimpleHt::new(10);

        let v = ht.remove(&String::from("abc"));
        assert_eq!(Option::None, v);
    }

    #[test]
    fn add_entry_same_bucket() {
        let mut ht: SimpleHt<String, i64> = SimpleHt::new(10);

        ht.add(String::from("abc"), 123);
        ht.add(String::from("def"), 124);

        let v = ht.get(&String::from("def")).unwrap();
        assert_eq!(&124, v);

        let v = ht.get(&String::from("abc")).unwrap();
        assert_eq!(&123, v);

        let v = ht.get(&"def".to_string()).unwrap();
        assert_eq!(&124, v);

        let v = ht.get(&String::from("aaa"));
        assert_eq!(Option::None, v);
    }

    #[test]
    fn add_entry_overflow_bucket() {
        let mut ht: SimpleHt<String, i64> = SimpleHt::new(10);

        ht.add(String::from("aaaaaaaaaab"), 123);
        ht.add(String::from("c"), 124);


        let bucket = &ht.buckets[1];
        assert_eq!(2, bucket.values.len());
        let first = &bucket.values[0];
        let second = &bucket.values[1];

        assert_eq!("aaaaaaaaaab", &first.0);
        assert_eq!("c", &second.0);

        assert_eq!(&123, &first.1);
        assert_eq!(&124, &second.1);
    }
}
