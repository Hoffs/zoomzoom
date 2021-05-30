pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    let k = k as usize;

    // ###########################################
    // 1600 ms
    // "normal" manual way
    // from 0 to len minus the window size +1, so if it has 10 elements and window is 9, it will do
    // 2 iterations. if 10 elements and window 10 it will do 1 iteration
    // for i in 0..((nums.len() - k) + 1) {
    //     let window = &nums[i..i+k];


    //     let mut max = std::i32::MIN;
    //     for x in window {
    //         if x > &max {
    //             max = *x;
    //         }
    //     }

    //     out.push(max);
    // }

    // ###########################################
    // 1608 ms
    // "rust" way (iter is pretty slow, replacing with manual max lookup is pretty much same as
    // above)
    // for x in nums.windows(k) {
    //     out.push(*x.iter().max().unwrap());
    // }

    // ###########################################
    // 700 ms
    // alternative way, keep track of max and max pos in the current "window"
    // let initial = &nums[..k];
    // let mut max = std::i32::MIN;
    // let mut max_i: usize = 0;
    // for i in 0..initial.len() {
    //     if initial[i] >= max {
    //         max = initial[i];
    //         max_i = i;
    //     }
    // }

    // out.push(max);

    // // len 10, window 9,
    // // 9..10 => 1 iter
    // for i in k..nums.len() {
    //     if nums[i] >= max {
    //         max = nums[i];
    //         max_i = i;
    //     }

    //     let window_start: usize = i - k + 1; // 9 - 9 + 1 = 1
    //     // if max_i was outside the window, recalculate max inside current
    //     // this part is basically the most inefficient
    //     if max_i < window_start {
    //         let window = &nums[window_start..i+1];

    //         assert_eq!(window.len(), k);

    //         let mut li = 0;
    //         let mut lmax = std::i32::MIN;
    //         for i in 0..window.len() {
    //             if window[i] >= lmax {
    //                 lmax = window[i];
    //                 li = i;
    //             }
    //         }

    //         max = lmax;
    //         max_i = window_start + li;
    //     }

    //     out.push(max);
    // }

    // ###########################################
    // 72ms
    // "optimal" with deque
    // idea is similar to above, but using double ended queue to more efficiently manage the most
    // relevant numbers.
    // - Every iteration check if head in the queue is not out of bounds for current window.
    // - Every iteration check if current number is bigger than tail of the queue. This gets rid of
    // all the older elements that are still in the queue (relevant for the current window) but are
    // smaller than current. Because current element comes later than previous ones, we can safely
    // discard all previous elements and safely assume that for current window and any further
    // window those previous elements are irrelevant and current element is max.
    // - Every iteration push current element index to the tail of the queue.
    // - Every iteration after initial window is visited, push to output the head of the queue.
    let mut q = std::collections::VecDeque::new();
    for i in 0..nums.len() {
        // pop everything from front that is no longer relevant to the current window
        while !q.is_empty() && i + 1 >= k && *q.front().unwrap() < i + 1 - k {
            q.pop_front();
        }

        // pop everything from back that is of lesser value than current element
        while !q.is_empty() && nums[*q.back().unwrap()] <= nums[i] {
            q.pop_back();
        }

        q.push_back(i);

        // once we iterate over first window keep pushing to output first in queue
        if i >= k -1 {
            out.push(nums[*q.front().unwrap()]);
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::max_sliding_window;
    #[test]
    fn it_works() {
        assert_eq!(max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3), vec![3,3,5,5,6,7]);
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
        assert_eq!(max_sliding_window(vec![1, -1], 1), vec![1, -1]);
        assert_eq!(max_sliding_window(vec![1, -1], 2), vec![1]);
    }
}
