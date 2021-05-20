pub trait SortKey {
    fn key(&self) -> i64;
}

// use last element for partition
pub fn quicksort<T: SortKey>(vec: &mut Vec<T>) {
    sort_partition(vec, 0, vec.len())

}

fn sort_partition<T: SortKey>(vec: &mut Vec<T>, from: usize, to: usize) {
    // if partition is 1 element or less, return
    if (to - from) <= 1 {
        return
    }

    // moves everything to the front and then puts
    // partition key at the last position
    let pivot = vec[to-1].key();
    let mut pos = from;
    for i in from..to {
        let key = vec[i].key();
        if key < pivot {
            vec.swap(pos, i);
            pos += 1;
        }
    }

    vec.swap(pos, to-1);

    sort_partition(vec, from, pos);
    sort_partition(vec, pos, to);
}

#[cfg(test)]
mod tests {
    use super::{quicksort, SortKey};
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

        quicksort(&mut v);


        let mut last_el = v[0];
        for x in v.iter().skip(1) {
            assert!(last_el <= *x);
            last_el = *x;
        }
    }

    #[test]
    fn sorts_negative_array() {
        let mut v = vec![-1, -43, 13, -36, 77];

        quicksort(&mut v);

        assert_eq!(v, vec![-43, -36, -1, 13, 77]);
    }

    #[test]
    fn sorts_positive_array() {
        let mut v = vec![1, 43, 13, 36, 77];

        quicksort(&mut v);

        assert_eq!(v, vec![1, 13, 36, 43, 77]);
    }
}
