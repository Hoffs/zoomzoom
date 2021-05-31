pub fn trap(height: Vec<i32>) -> i32 {
    let mut max_h = 0;
    let mut sum = 0;
    let mut visited = Vec::new();
    visited.resize(height.len(), 0);

    // basically goes from left to right, keeps track of max height left, when it encounters
    // another height it iterates left until same or higher height is reached. while iterating left
    // it also keeps track of additions that already happened, thus not requiring additional O(n)
    // operation to get a sum of all trap heights. worst case is around O(n^2) in case when
    // height[0] is max height and then all elements to the right start from 0 and keep increasing
    // by one, this way requring for each one to go back to the very start.
    //
    //
    // ALTERNATIVE, more efficient way is to move from left and right at the same time, and keeping
    // the max bounds. Which is similarly to what I did here, just for both sides. The idea is to
    // keep left bound and right bound, every iteration move the bound which is lower to the
    // opposite side. This (again) works the same way as above, as you can guarantee that for that
    // height there will be a bound on another side.

    for x in 0..height.len() {
        let current_h = height[x];
        if current_h == 0 {
            continue;
        }

        if max_h > 0 {
            let usable_h = std::cmp::min(max_h, current_h);
            for xb in (0..x).rev() {
                if height[xb] >= usable_h {
                    break;
                }

                let max_add = usable_h - height[xb];
                sum += max_add - visited[xb];
                visited[xb] = max_add;
            }
        }


        if current_h > max_h {
            max_h = current_h;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::trap;

    #[test]
    fn it_works() {
        assert_eq!(trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    }
}
