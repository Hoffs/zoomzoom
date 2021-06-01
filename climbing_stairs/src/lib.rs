pub fn climb_stairs(n: i32) -> i32 {
    let mut steps = Vec::new();
    steps.resize(n as usize + 1, 0);
    steps[0] = 1;

    // idea is that steps[..] tracks how many possible ways it is possible to get to that position
    // since 2 possibilities are 1 or 2 steps, so for each i we add how many ways there are to get
    // to i-1 and how many ways there are to get to i-2.
    // So if we are at 5th step, we check how many ways we could get to 4th step and add those to
    // 5th pos, then we check how many ways we could get to 3th step and add those to 5th pos.

    for i in 1..steps.len() {
        let one_step = i as i32 - 1;
        let two_step = i as i32 - 2;
        if one_step >= 0 {
            steps[i] += steps[one_step as usize];
        }
        if two_step >= 0 {
            steps[i] += steps[two_step as usize];
        }
    }


    steps[steps.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::climb_stairs;

    #[test]
    fn it_works() {
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
    }
}
