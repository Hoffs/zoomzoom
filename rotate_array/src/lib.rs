pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let abs_rot = k as usize % nums.len();
    if nums.len() == 1 || abs_rot == 0 {
        return;
    }

    // go around swapping numbers, while keeping last one in memory
    // until start is reached.
    // So for [1, 2, 3] with rot 2,
    // [1, 2, 3] (1)
    // [_, 2, 1] (3)
    // [_, 3, 1] (2)
    // [2, 3, 1] end
    fn swap_rec(n: &mut Vec<i32>, start: usize, curr: usize, rot: usize, last_num: i32, visited: usize) -> usize {
        let mv_to = (curr + rot) % n.len();
        let temp = n[mv_to];
        n[mv_to] = last_num;
        if mv_to != start {
            return swap_rec(n, start, mv_to, rot, temp, visited+1);
        }

        return visited;
    }

    // do swap for amount of times that rotate is needed, but end if we swapped enough times
    // already.
    // e.g.
    // [1, 2, 3, 4] will do swap_rec(..) 2 times for result of:
    // [3, 2, 1, 4] then move index by one and do two more time for result of:
    // [3, 4, 1, 2]
    let mut visited = 0;
    for x in 0..abs_rot {
        visited = swap_rec(nums, x, x, abs_rot, nums[x], visited+1);
        if visited == nums.len() {
            return;
        }
    }


    // better method is to reverse part until new start,
    // then reverse part from new start till end
    // and then reverse whole thing
    // so [1, 2, 3, 4, 5] with rot 3, start would be at 3
    // [2, 1 | 5, 4, 3]
    // [3, 4, 5, 1, 2]
}

#[cfg(test)]
mod tests {
    use super::rotate;

    #[test]
    fn it_works() {
        let mut a = vec![1,2,3,4,5,6,7];
        rotate(&mut a, 3);
        assert_eq!(a, vec![5,6,7,1,2,3,4]);

        let mut b = vec![-1,-100,3,99];
        rotate(&mut b, 2);
        assert_eq!(b, vec![3,99,-1,-100]);

        let mut b2 = vec![-1,-100,3,99];
        rotate(&mut b2, 3);
        assert_eq!(b2, vec![-100,3,99,-1]);

        let mut b1 = vec![-1,-100,3,99,5];
        rotate(&mut b1, 2);
        assert_eq!(b1, vec![99,5,-1,-100, 3]);

        let mut c = vec![-1,-100,3,99];
        rotate(&mut c, 4);
        assert_eq!(c, vec![-1,-100,3,99]);

        let mut d = vec![3];
        rotate(&mut d, 7);
        assert_eq!(d, vec![3]);
    }
}
