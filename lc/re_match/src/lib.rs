// leetcode 10
// has to match full string
// Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*' where:
// '.' Matches any single character.
// '*' Matches zero or more of the preceding element.

#[derive(Debug)]
struct StarMatch {
    ch: char,
    is_any: bool,
    str_p: usize,
    pt_p: usize,
}

pub fn is_match(input: String, pattern: String) -> bool {
    let mut str_p = 0; // string pointer
    let mut pt_p = 0; // pattern pointer
    let mut star_match: Option<StarMatch> = None; //

    let input: Vec<_> = input.chars().collect();
    let pattern: Vec<_> = pattern.chars().collect();

    if pattern.len() == 0 {
        return input.len() == 0;
    }

    loop {
        if let Some(in_c) = input.get(str_p) {
            // if we still have some pattern to read
            if let Some(pt_c) = pattern.get(pt_p) {
                if  pt_c == in_c || pt_c == &'.' {
                    // match direct value
                    str_p += 1;
                    pt_p += 1;
                    continue;
                } else if pt_c == &'*' {
                    // we have star in pattern, matches anything any amount of times
                    // we just mark this position as we can always reset to this
                    // and increase pattern pointer and try matching whatever comes after star
                    let prev_c = pattern.get(pt_p - 1).unwrap();

                    star_match = Some(StarMatch{ch: *prev_c, is_any: prev_c == &'.', str_p, pt_p});
                    pt_p += 1;
                    continue;
                } else {
                    // didnt match anything, check if next symbol is star and skip
                    if let Some('*') = pattern.get(pt_p + 1) {
                        pt_p += 2;
                        continue;
                    }
                }
            }

            if let Some(StarMatch{ch, is_any, str_p: string_pos, pt_p: pattern_pos}) = star_match {
                // nothing matched, but we have star
                // reset back to where we encountered star + 1
                // increase the string position by 1
                if in_c != &ch && !is_any {
                    println!("str_p {}, pt_p {}, in_c {}", str_p, pt_p, in_c);
                    println!("str_p {}, pt_p {}, in_c {}", str_p, pt_p, in_c);
                    println!("{:#?}", star_match);
                    println!("false in some");
                    // star_match = None;
                    // pt_p += 2;
                    return false;
                }

                str_p = string_pos+1;
                pt_p = pattern_pos+1;
                star_match = Some(StarMatch{ch, is_any, str_p: string_pos+1, pt_p: pattern_pos});
            } else {
                    println!("false in else");
                return false;
            }
        } else {
            break;
        }
    }


    println!("str_p {}, pt_p {}, pattern {:#?}", str_p, pt_p, pattern);
    // if we ended on star pattern, skip over (e.g. aabb matching with aabb*), since
    // the rest of the string matched, just it didnt finish on the end of the pattern ("b*")
    if let Some('*') = pattern.get(pt_p) {
        pt_p += 1;
    }

    // "drain" star pattern if we exceed input length
    while let Some('*') = pattern.get(pt_p + 1) {
        pt_p += 2;
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
        assert!(is_match("abbbb".to_string(), "ab*".to_string()));
        assert!(is_match("ddddf".to_string(), ".*df".to_string()));
        assert!(is_match("aaaaf".to_string(), "a*f".to_string()));
        assert!(is_match("aaaaf".to_string(), "a.*".to_string()));
        assert!(is_match("aaaaf".to_string(), ".*".to_string()));
        assert!(!is_match("aaaaf".to_string(), "a*zf".to_string()));
        assert!(!is_match("abcdf".to_string(), ".*zf".to_string()));
        assert!(!is_match("abcdf".to_string(), "az*f".to_string()));
        assert!(!is_match("abcdf".to_string(), "az*".to_string()));
    }

    #[test]
    fn matches_single() {
        assert!(is_match("abcdf".to_string(), "abcd.".to_string()));
        assert!(is_match("abcdf".to_string(), ".bcdf".to_string()));
        assert!(is_match("abcdf".to_string(), "ab.df".to_string()));
        assert!(is_match("abcdf".to_string(), ".....".to_string()));
        assert!(!is_match("abcdf".to_string(), "abcdf.".to_string()));
        assert!(!is_match("abcdf".to_string(), "abc.".to_string()));
        assert!(!is_match("abcdf".to_string(), ".bcdfe".to_string()));
        assert!(!is_match("abcdf".to_string(), ".bcd".to_string()));
    }

    #[test]
    fn matches_edgecase() {
        assert!(is_match("aaaaaaaaaaab".to_string(), ".*ab".to_string()));
        assert!(is_match("z".to_string(), "a*b*c*d*e*f*g*h*z".to_string()));
        assert!(is_match("".to_string(), "a*".to_string()));
        assert!(is_match("aabb".to_string(), "aabb*".to_string()));
        assert!(is_match("aabb".to_string(), "aabb*z*f*g*.*".to_string()));
        assert!(is_match("mississipi".to_string(), "mis*is*ip*.".to_string()));
        assert!(is_match("mississippi".to_string(), "mis*is*ip*.".to_string())); // this fails
        assert!(is_match("miiiiiiiipiiiii".to_string(), "mi*....i*..".to_string()));
    }
}
