pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut min_len = std::usize::MAX;
    let mut start_seq_idx = 0;
    let mut seq_sum = 0;

    // idea - go from 0 to nums.len() increasing the sum, if sum is more or equal,
    // try decreasing the length from left side until its less than target.
    for i in 0..nums.len() {
        seq_sum += nums[i];

        while seq_sum >= target && start_seq_idx < nums.len() {
            if seq_sum >= target && (i - start_seq_idx) + 1 < min_len {
                min_len = (i - start_seq_idx) + 1;
            }

            seq_sum -= nums[start_seq_idx];
            start_seq_idx += 1;
        }
    }

    if min_len == std::usize::MAX {
        0
    } else {
        min_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::min_sub_array_len;

    #[test]
    fn it_works() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
        assert_eq!(min_sub_array_len(11, vec![1,2,3,4,5]), 3);
    }
}
