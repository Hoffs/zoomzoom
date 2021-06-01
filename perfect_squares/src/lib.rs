pub fn num_squares(n: i32) -> i32 {
    let mut sq = Vec::new();
    let max_sq = (n as f32).sqrt().floor() as i32;
    for i in 1..=max_sq {
        sq.push(i*i);
    }

    let mut visited = std::collections::HashSet::new();
    let mut least_steps = n;

    // This is sort of DFS, but ideal solution would be with BFS, since we visit wide and don't go
    // too deep. And once we reach N it will be shortest path, because of BFS.
    fn rec_pos(nums: &Vec<i32>, steps: i32, rem: i32, visited: &mut std::collections::HashSet<(i32, i32)>, least_steps: &mut i32) {
        // if we already have a solution with less or equal steps, return.
        if steps >= *least_steps {
            return;
        }

        for x in nums.iter().rev() {
            if x <= &rem {
                let steps_next = steps + 1;
                let rem_next = rem - x;

                if rem_next == 0 {
                    if steps_next < *least_steps {
                        *least_steps = steps_next;
                    }
                    return;
                }

                // dont go down a path if we already had same amount of steps with same remainder
                if visited.insert((steps_next, rem_next)) {
                    rec_pos(nums, steps_next, rem_next, visited, least_steps);
                }
            }
        }
    }


    rec_pos(&sq, 0, n, &mut visited, &mut least_steps);

    least_steps
}

#[cfg(test)]
mod tests {
    use super::num_squares;

    #[test]
    fn it_works() {
        assert_eq!(num_squares(12), 3); // 4 + 4 + 4
        assert_eq!(num_squares(9), 1); // 9
        assert_eq!(num_squares(11), 3); // 9 + 1 + 1
        assert_eq!(num_squares(13), 2); // 9 + 4
        assert_eq!(num_squares(309), 3); //
        assert_eq!(num_squares(6255), 4); //
        assert_eq!(num_squares(48), 3); //
    }
}
