pub trait SortKey {
    fn key(&self) -> i64;
}

// split into chunks until size=1,
// then on merge build single vec in sorted fashion which is just O(n)
pub fn mergesort<T: SortKey + std::fmt::Debug>(mut vec: Vec<T>) -> Vec<T> {
    if vec.len() == 1 {
        return vec;
    }

    let mut left = Vec::with_capacity(vec.len() / 2);
    let mut right = Vec::with_capacity(vec.len() / 2);

    let middle = vec.len() / 2;
    for i in 0..vec.len() {
        if let Some(el) = vec.pop() {
            if i < middle {
                left.push(el);
            } else {
                right.push(el);
            }
        }
    }

    let left = mergesort(left);
    let right = mergesort(right);
    merge(left, right)
}

// merge two sorted vec
fn merge<T: SortKey + std::fmt::Debug>(mut vec1: Vec<T>, mut vec2: Vec<T>) -> Vec<T> {
    let mut merged = Vec::with_capacity(vec1.len() + vec2.len());

    let mut v1drain = vec1.drain(..);
    let mut v2drain = vec2.drain(..);
    let mut pair = (v1drain.next(), v2drain.next());
    loop {
        pair = match pair {
            (Some(e1), Some(e2)) => {
                if e1.key() > e2.key() {
                    merged.push(e2);
                    (Some(e1), v2drain.next())
                } else {
                    merged.push(e1);
                    (v1drain.next(), Some(e2))
                }
            },
            (None, Some(e2)) => {
                merged.push(e2);
                (None, v2drain.next())
            },
            (Some(e1), None) => {
                merged.push(e1);
                (v1drain.next(), None)
            },
            (None, None) => {
                break;
            }
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::{mergesort, SortKey};
    use rand::random;

    impl SortKey for i64 {
        fn key(&self) -> i64 {
            *self
        }
    }

    #[test]
    fn sorts_random_array() {
        let mut v = Vec::new();
        v.resize_with(200, || random::<i64>());

        let v = mergesort(v);

        let mut last_el = v[0];
        for x in v.iter().skip(1) {
            assert!(last_el <= *x);
            last_el = *x;
        }
    }

    #[test]
    fn sorts_negative_array() {
        let v = vec![-1, -43, 13, -36, 77];

        let v = mergesort(v);

        assert_eq!(v, vec![-43, -36, -1, 13, 77]);
    }

    #[test]
    fn sorts_positive_array() {
        let v = vec![1, 43, 13, 36, 77];

        let v = mergesort(v);

        assert_eq!(v, vec![1, 13, 36, 43, 77]);
    }
}
