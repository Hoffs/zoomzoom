// leetcode 44
// has to match full string
pub fn is_match(input: String, pattern: String) -> bool {
    // 4 states
    // - matches exact or ?
    // - matches *
    // - matches nothing but is inside *
    // - matches nothing
    let mut str_p = 0; // string pointer
    let mut pt_p = 0; // pattern pointer
    let mut star_reset: Option<(usize, usize)> = None; // point where we left * matching


    let input: Vec<_> = input.chars().collect();
    let pattern: Vec<_> = pattern.chars().collect();

    if pattern.len() == 0 {
        return input.len() == 0;
    }

    loop {
        if let Some(in_c) = input.get(str_p) {
            // if we still have some pattern to read
            if let Some(pt_c) = pattern.get(pt_p) {
                if  pt_c == in_c || pt_c == &'?' {
                    // match direct value
                    str_p += 1;
                    pt_p += 1;
                    continue;
                } else if pt_c == &'*' {
                    // we have star in pattern, matches anything any amount of times
                    // we just mark this position as we can always reset to this
                    // and increase pattern pointer and try matching whatever comes after star
                    star_reset = Some((str_p, pt_p));
                    pt_p += 1;
                    continue;
                }
            }

            if let Some((string_pos, pattern_pos)) = star_reset {
                // nothing matched, but we have star
                // reset back to where we encountered star + 1
                // increase the string position by 1
                str_p = string_pos+1;
                pt_p = pattern_pos+1;
                star_reset = Some((string_pos+1, pattern_pos));
            } else {
                return false;
            }
        } else {
            break;
        }
    }

    // "drain" star pattern if we exceed input length
    while let Some('*') = pattern.get(pt_p) {
        pt_p += 1;
    }

    str_p == input.len() && pt_p == pattern.len()
}


#[cfg(test)]
mod tests {
    use super::is_match;
    #[test]
    fn matches_exact() {
        assert!(is_match("abcdf".to_string(), "abcdf".to_string()));
        assert!(!is_match("abcdf".to_string(), "abcd".to_string()));
        assert!(!is_match("abcdf".to_string(), "bcdf".to_string()));
    }

    #[test]
    fn matches_sequence() {
        assert!(is_match("abcdf".to_string(), "ab*".to_string()));
        assert!(is_match("abcdf".to_string(), "*df".to_string()));
        assert!(is_match("abcdf".to_string(), "a*f".to_string()));
        assert!(!is_match("abcdf".to_string(), "a*zf".to_string()));
        assert!(!is_match("abcdf".to_string(), "*zf".to_string()));
        assert!(!is_match("abcdf".to_string(), "az*f".to_string()));
        assert!(!is_match("abcdf".to_string(), "az*".to_string()));
    }

    #[test]
    fn matches_single() {
        assert!(is_match("abcdf".to_string(), "abcd?".to_string()));
        assert!(is_match("abcdf".to_string(), "?bcdf".to_string()));
        assert!(is_match("abcdf".to_string(), "ab?df".to_string()));
        assert!(!is_match("abcdf".to_string(), "abcdf?".to_string()));
        assert!(!is_match("abcdf".to_string(), "abc?".to_string()));
        assert!(!is_match("abcdf".to_string(), "?bcdfe".to_string()));
        assert!(!is_match("abcdf".to_string(), "?bcd".to_string()));
    }

    #[test]
    fn matches_edgecase() {
        assert!(is_match("aaabba".to_string(), "a*ba".to_string()));
        assert!(is_match("aaaaaaaaaaab".to_string(), "*ab".to_string()));
        assert!(is_match("aabb".to_string(), "********".to_string()));
        assert!(is_match("".to_string(), "********".to_string()));
        assert!(is_match("aabb".to_string(), "aabb****".to_string()));
    }
}
