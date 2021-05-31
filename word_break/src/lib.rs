pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut words = std::collections::HashMap::new();
    for x in word_dict {
        words.insert(x, 0 as i8);
    }

    // create vec for storing found words
    // [t, f, f, f, ,f], where len = string+1, 0 marks the start and is always true
    // when iterating over the string, every iteration try going backwards until
    // last position where word ended is reached, try to find a word from last word end until
    // current position. If found, then we found another fitting word and we can move along. If not
    // found, then try going backwards until end is reached. This way a vec is filled in a way
    // that, if all the true only is marked if sequence until is also true, as in possible to make
    // using provided words.
    let mut dp = Vec::new();
    dp.resize(s.len()+1, false);
    dp[0] = true;

    // alternative method would be to start iterating from 0, then check if any of the words match
    // at pos 0, if a word matches from pos 0, push to stack/queue/max-heap/whatever the position
    // where it would end. Once all words are checked at pos 0, pop from the structure next
    // position and check all words again, pushing positions back to the structure. If current
    // pos+word len = string len, then we have built a full string out of words.

    for i in 0..s.len() {
        for j in (0..=i).rev() {
            if dp[j] {
                let slice = &s[j..=i];
                if words.contains_key(slice) {
                    dp[i+1] = true; // +1, because dp is offset by 1
                    break;
                }
            }
        }
    }

    // println!("{:#?}", dp);
    // if last element is true, means that it is possible to create a sequence
    // of words.
    dp[dp.len()-1]
}

// word break 1 solution using queue/dfs
pub fn word_break_alt(s: String, word_dict: Vec<String>) -> bool {
    let mut words = std::collections::HashMap::new();
    for x in word_dict {
        words.insert(x, 0 as i8);
    }

    // alternative method would be to start iterating from 0, then check if any of the words match
    // at pos 0, if a word matches from pos 0, push to stack/queue/max-heap/whatever the position
    // where it would end. Once all words are checked at pos 0, pop from the structure next
    // position and check all words again, pushing positions back to the structure. If current
    // pos+word len = string len, then we have built a full string out of words.
    let mut q = std::collections::VecDeque::new();
    q.push_back(0);

    let mut tried_from = std::collections::HashSet::new();

    while let Some(i) = q.pop_front() {
        let slice = &s[i..];
        for w in words.keys() {
            if slice.starts_with(w) {
                if i + w.len() == s.len() {
                    return true;
                }

                if tried_from.insert(i + w.len()) {
                    q.push_front(i + w.len());
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{word_break, word_break_alt};

    #[test]
    fn it_works() {
        assert_eq!(word_break("a".to_string(), vec!["a".to_string()]), true);
        assert_eq!(word_break("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]), true);
        assert_eq!(word_break("leetcode".to_string(), vec!["le".to_string(), "leet".to_string(), "code".to_string()]), true);
        assert_eq!(word_break("applepenapple".to_string(), vec!["apple".to_string(), "pen".to_string()]), true);
        assert_eq!(word_break("catsandog".to_string(), vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]), false);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(word_break_alt("a".to_string(), vec!["a".to_string()]), true);
        assert_eq!(word_break_alt("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]), true);
        assert_eq!(word_break_alt("leetcode".to_string(), vec!["le".to_string(), "leet".to_string(), "code".to_string()]), true);
        assert_eq!(word_break_alt("applepenapple".to_string(), vec!["apple".to_string(), "pen".to_string()]), true);
        assert_eq!(word_break_alt("catsandog".to_string(), vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]), false);
    }
}
