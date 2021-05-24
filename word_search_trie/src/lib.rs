use std::collections::HashMap;

// 212 word search 2
// initial:
// Runtime: 916 ms, faster than 7.41% of Rust online submissions for Word Search II.
// Memory Usage: 2.4 MB, less than 11.11% of Rust online submissions for Word Search II.
// with vec:
// Runtime: 744 ms, faster than 11.11% of Rust online submissions for Word Search II.
// Memory Usage: 2.8 MB, less than 7.41% of Rust online submissions for Word Search II.
// recursive:
// 860 ms	2.8 MB
// with some optimizations:
// 380 ms	2.8 MB

#[derive(Debug)]
struct TrieNode {
    letters: Vec<Option<TrieNode>>,
    word: Option<String>,
}

impl TrieNode {
    fn get_child(&self, ch: char) -> &Option<TrieNode> {
        let i = (ch as u8 - b'a') as usize;
        &self.letters[i]
    }
}

fn as_idx(ch: char) -> usize {
    (ch as u8 - b'a') as usize
}

impl Default for TrieNode {
    fn default() -> Self {
        let mut v: Vec<Option<TrieNode>> = Vec::new();
        v.resize_with(26, Default::default);
        TrieNode{letters: v, word: None}
    }
}

fn build_trie(words: Vec<String>) -> TrieNode {
    let mut root: TrieNode = Default::default();

    for word in words {
        let mut node = &mut root;
        for c in word.chars() {
            let c_idx = as_idx(c);
            if let None = &node.letters[c_idx] {
                node.letters[c_idx] = Some(Default::default());
            }

            let w = &mut node.letters[c_idx];
            node = w.as_mut().unwrap();
        }

        node.word = Some(word.clone());
    }

    root
}

pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let words_len = words.len();
    let trie = build_trie(words);
    // println!("board {:?}\ntrie {:#?}", board, trie);

    // [[r, o, w], [r, o, w]]
    let mut results = HashMap::new();
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if results.len() == words_len {
                break;
            }

            if let Some(node) = trie.get_child(board[y][x]) {
                scan_for_words(&board, x, y, node, &mut results, words_len);
            }
        }
    }

    results.keys().map(|x| x.clone()).collect()
}

fn scan_for_words(board: &Vec<Vec<char>>, mut x: usize, mut y: usize, trie: &TrieNode, results: &mut HashMap<String, bool>, max_results: usize) {
    // marks already visited nodes
    let mut visited: Vec<_> = Vec::new();

    let mut visited_level_sides: Vec<usize> = Vec::new();
    visited_level_sides.resize_with(15, Default::default);

    let mut level = 1;
    visited.push((x, y));

    let mut trie_stack: Vec<&TrieNode> = Vec::new();
    let mut trie_node = trie;

    'word: loop {
        if let Some(w) = &trie_node.word {
            results.insert(w.clone(), true);
        }

        if results.len() == max_results {
            return;
        }

        for side in visited_level_sides[level]..4 {
            visited_level_sides[level] += 1;
            let (lx, ly) = match side {
                0 if x > 0 => (x-1, y),
                0 => continue,
                1 if y > 0 => (x, y-1),
                1 => continue,
                2 => (x+1, y),
                3 => (x, y+1),
                _ => break,
            };

            if visited.contains(&(lx, ly)) {
                    continue;
            }

            if let Some(yel) = board.get(ly) {
                if let Some(xel) = yel.get(lx) {
                    if let Some(next_node) = trie_node.get_child(*xel) {
                        x = lx;
                        y = ly;
                        visited.push((x, y));
                        trie_stack.push(trie_node);
                        trie_node = next_node;
                        level += 1;
                        continue 'word;
                    }
                }
            }
        }

        if level > 1 && visited_level_sides[level] == 4 {
            // all sides exhausted for current level, try going level back.
            // reset current level sides
            visited_level_sides[level] = 0;
            level -= 1;
            visited.pop();
            let (last_x, last_y) = visited.last().unwrap();
            if let Some(prev_trie) = trie_stack.pop() {
                trie_node = prev_trie;
            } else {
                trie_node = trie;
            }
            x = *last_x;
            y = *last_y;
            continue;
        }

        break;
    }
}

fn scan_for_words_rec(board: &Vec<Vec<char>>, x: usize, y: usize, trie: &TrieNode, mut visited: Vec<(usize, usize)>, results: &mut HashMap<String, bool>) {
    visited.push((x, y));

    if let Some(w) = &trie.word {
        results.insert(w.clone(), true);
    }

    if trie.letters.len() == 0 {
        return;
    }

    for side in 0..4 {
        let (lx, ly) = match side {
            0 if x > 0 => (x-1, y),
            0 => (x, y),
            1 if y > 0 => (x, y-1),
            1 => (x, y),
            2 => (x+1, y),
            3 => (x, y+1),
            _ => break,
        };

        // handles usize underflow cases
        if lx == x && ly == y {
            continue;
        }

        if visited.contains(&(lx, ly)) {
                continue;
        }

        if let Some(yel) = board.get(ly) {
            if let Some(xel) = yel.get(lx) {
                if let Some(next_node) = trie.get_child(*xel) {
                    let visited = visited.clone();
                    scan_for_words_rec(board, lx, ly, next_node, visited, results);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::find_words;

    #[test]
    fn it_works() {
        let map = vec![vec!['a', 'b', 'c', 'e'], vec!['s', 'f', 'c', 's'], vec!['a', 'd', 'e', 'e']];
        assert_eq!(vec!["abcced".to_string(), "see".to_string()], find_words(map.clone(), vec!["abcced".to_string(), "see".to_string(), "abcb".to_string()]));

        assert_eq!(vec!["a".to_string()], find_words(vec![vec!['a']], vec!["a".to_string()]));
    }
}
