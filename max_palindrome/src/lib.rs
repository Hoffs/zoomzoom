// 64ms (faster than 48.67% of Rust online submissions)
// dynamic programming longest palindrome
pub fn longest_palindrome(s: String) -> String {
    if s.len() == 1 {
        return s
    }

    let mut map: Vec<Vec<bool>> = Vec::new();
    let chars: Vec<char> = s.chars().collect();

    let mut p_i = 0;
    let mut p_len = 1;
    // iterate from the end of the string
    // every iteration go one char back and iterate till the end of the string
    //

    //   b a b      b a b
    // b t f t  =>  t   t
    // a _ t f        t
    // b _ _ t
    map.resize_with(s.len(), || {
        let mut x = Vec::new();
        x.resize(s.len(), false);
        x
    });


    for i in (0..chars.len()).rev() {
        // i = row, basically which character we are using as a base
        map[i][i] = true; // set middle to true, since single char is palindrome

        for j in i+1..chars.len() {
            // j = column, which character we are trying to compare with
            // println!("comparing {} == {}", chars[i], chars[j]);
            if chars[i] == chars[j] {
                // matches, if diff is 1, they are sides by sides, otherwise check
                // if in the row below and column to the left is true, then we can
                // check this position as also true
                // otherwise, just check every match without this if, and then look
                // for longest triangle in the map.
                // println!("len {}, i+1,j-1 {}", j-i, map[i+1][j-1]);
                if j - i == 1 || map[i+1][j-1] {
                    map[i][j] = true;

                    if j - i + 1 > p_len {
                        p_len = j - i + 1;
                        p_i = i;
                    }
                }
            }
        }
    }

    // println!("{:#?}", map);
    chars[p_i..p_i+p_len].iter().collect()
}

#[cfg(test)]
mod tests {
    use super::longest_palindrome;

    #[test]
    fn it_works() {
        assert_eq!(longest_palindrome("babad".to_string()), "aba".to_string());
        assert_eq!(longest_palindrome("bacab".to_string()), "bacab".to_string());
        assert_eq!(longest_palindrome("abc".to_string()), "a".to_string());
    }
}
