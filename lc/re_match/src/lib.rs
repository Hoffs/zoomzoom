// leetcode 10
// has to match full string
// Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*' where:
// '.' Matches any single character.
// '*' Matches zero or more of the preceding element.
#[derive(Debug, PartialEq, Eq, Clone)]
struct Match {
    ch: char,
    is_seq: bool,
    i_ptr: usize,
    extended: usize,
    p_ptr: usize,
}

impl Match {
    fn matches(&self, compare: &char) -> bool {
        self.ch == '.' || &self.ch == compare
    }

    fn len(&self) -> usize {
        if self.is_seq {
            2
        } else {
            1
        }
    }
}

pub fn is_match(input: String, pattern: String) -> bool {
    // idea is to push star matches to stack and then eagerly move to next pattern,
    // if matching then fails at some point, pop the stack, increase and reset
    // this way its pretty easy to move back multiple matches
    // if stack is empty or we can't move forward with last element => return false
    //
    // this is not as optimal as dp solution, but it is pretty fun way to implement it
    let mut backtrack_stack: Vec<Match> = Vec::new();

    let input: Vec<_> = input.chars().collect();
    let pattern: Vec<_> = pattern.chars().collect();

    let mut p_ptr = 0;
    let mut i_ptr = 0;

    'main: loop {
        // print!("\nloop with p_ptr {}, i_ptr {}\nbacktrack {:?}\n", p_ptr, i_ptr, backtrack_stack);
        let mut p_match: Option<_> = None;
        if let Some(matching_ch) = pattern.get(p_ptr) {
            if let Some('*') = pattern.get(p_ptr + 1) {
                let seq_match = Match{ch: *matching_ch, is_seq: true, i_ptr, extended: 0, p_ptr};
                backtrack_stack.push(seq_match.clone());
                p_match = Option::Some(seq_match);
            } else {
                p_match = Option::Some(Match{ch: *matching_ch, is_seq: false, i_ptr, extended: 0, p_ptr});
            }
        }

        let ch = input.get(i_ptr);
        if ch == None {
            if p_ptr == pattern.len() {
                break;
            }
        }

        if let Some(current_match) = p_match {
            if let Some(ch) = ch {
                if current_match.matches(ch) {
                    p_ptr += current_match.len();
                    i_ptr += 1;
                    continue;
                }
            }

            if current_match.is_seq {
                // if we skip over it, pop it off from backtrack
                backtrack_stack.pop();
                p_ptr += current_match.len();
                continue;
            }
        }

        // println!("trying to backtrack");
        while let Some(mut last_seq) = backtrack_stack.pop() {
            // println!("got backtrack to {:?}", last_seq);
            if let Some(next_ch) = input.get(last_seq.i_ptr + last_seq.extended + 1) {
                // println!("got backtrack char {}", next_ch);
                if last_seq.matches(next_ch) {
                    // we can extend last match by 1
                    last_seq.extended += 1;
                    i_ptr = last_seq.i_ptr + last_seq.extended + 1;
                    p_ptr = last_seq.p_ptr + last_seq.len();
                    backtrack_stack.push(last_seq);
                    continue 'main;
                }
            }

            // try the case where we skip the match instead
            i_ptr = last_seq.i_ptr;
            p_ptr = last_seq.p_ptr + last_seq.len();
            continue 'main;
        }

        // nowhere to backtrack to
        break;
    }

    // print!("\n\ninput {:?}\npattern {:?}\np_ptr {}, i_ptr {}\n", input, pattern, p_ptr, i_ptr);
    p_ptr == pattern.len() && i_ptr == input.len()
}

#[cfg(test)]
mod tests {
    use super::is_match;
    #[test]
    fn matches_exact() {
        assert!(is_match("abcdf".to_string(), "abcdf".to_string()));
        assert!(!is_match("abcdf".to_string(), "abcd".to_string()));
        assert!(!is_match("abcdf".to_string(), "bcdf".to_string()));
        assert!(!is_match("abcdf".to_string(), "abcdfeeee".to_string()));
        assert!(!is_match("abcdf".to_string(), "".to_string()));
        assert!(!is_match("".to_string(), "aaa".to_string()));
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
        assert!(is_match("mississippi".to_string(), "mis*is*ip*.".to_string()));
        assert!(is_match("miiiiiiiipiiiii".to_string(), "mi*....i*..".to_string()));
        assert!(!is_match("mississippi".to_string(), "mis*is*p*.".to_string()));
        assert!(is_match("bbbba".to_string(), ".*a*a".to_string()));
        assert!(is_match("bbbbaa".to_string(), ".*a*aa".to_string()));
    }
}
