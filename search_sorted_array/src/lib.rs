pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    // first find pivot point, then search in that
    // since both is just bin search, its O(2logn) => O(logn)
    fn find_pivot_rec(nums: &Vec<i32>, from: usize, to: usize) -> usize {
        if to - from == 1 {
            if nums[to] < nums[from] {
                return to;
            } else {
                return from;
            }
        }
        if to - from == 0 {
            return from;
        }

        let midpoint = from + ((to - from) / 2);
        let midval = nums[midpoint];
        if nums[from] > midval {
            // from is higher than mid
            return find_pivot_rec(nums, from, midpoint);
        } else if nums[to] < midval {
            // to is lower than mid
            return find_pivot_rec(nums, midpoint, to);
        } else {
            // from is not higher, to is not lower, sequence seems to be sorted
            from
        }
    }

    let pivot = find_pivot_rec(&nums, 0, nums.len() - 1);

    // it would probably be possible to do it in the single run in above recursion
    // like, if we are looking for 0 in [5,6,0,1,2,3,4], we check midpoint, we see that part from
    // middle till end is sorted/correct and is 1 to 4, but we are looking for 0, so it has to be
    // in other half, then in [5,6,0,1] we see that its not correct, and contains 5 to 1, so it has
    // to be somewhere there, splitting [5,6] and [6,0,1] we see that from to mid slice is correct,
    // but only has 5 to 6, so its not what we are looking for, so go with right, and so on. Should
    // be possible to implement something like that.
    fn bin_search_pivoted(
        target: i32,
        nums: &Vec<i32>,
        from: i32,
        to: i32,
        start_idx: usize,
    ) -> Option<usize> {
        if from > to {
            return None
        }

        let midpoint = from + ((to - from) / 2);
        let midval = nums[(start_idx + midpoint as usize) % nums.len()];

        if midval == target {
            return Some((start_idx + midpoint as usize) % nums.len());
        } else if midval > target {
            return bin_search_pivoted(target, nums, from, midpoint-1, start_idx);
        } else {
            return bin_search_pivoted(target, nums, midpoint+1, to, start_idx);
        }
    }

    let r = bin_search_pivoted(target, &nums, 0, nums.len() as i32 - 1, pivot);
    if let Some(rr) = r {
        return rr as i32;
    } else {
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::search;

    #[test]
    fn it_works() {
        assert_eq!(search(vec![4, 5, 6, 0, 1, 2, 3], 0), 3);
        assert_eq!(search(vec![6, 0, 1, 2, 3, 4, 5], 0), 1);
        assert_eq!(search(vec![5, 6, 0, 1, 2, 3, 4], 0), 2);
        assert_eq!(search(vec![0, 1, 2, 3, 4, 5, 6], 0), 0);
        assert_eq!(search(vec![2, 3, 4, 5, 6, 0, 1], 0), 5);
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(search(vec![1], 0), -1);
    }
}
