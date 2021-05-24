extern crate word_search_trie;

fn main() {
    let map = vec![vec!['a', 'b', 'c', 'e'], vec!['s', 'f', 'c', 's'], vec!['a', 'd', 'e', 'e']];
    word_search_trie::find_words(map.clone(), vec!["abcced".to_string(), "see".to_string(), "abcb".to_string()]);
}
