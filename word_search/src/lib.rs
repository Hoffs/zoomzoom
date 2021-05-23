pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    println!("board {:?}, word {}", board, word);
    let word: Vec<_> = word.chars().collect();

    // [[r, o, w], [r, o, w]]
    for (y, yv) in board.iter().enumerate() {
        for (x, xv) in yv.iter().enumerate() {
            if xv == &word[0] {
                if scan_for_word(&board, &word, x, y) {
                    return true
                }
            }
        }
    }

    false
}

fn scan_for_word(board: &Vec<Vec<char>>, word: &Vec<char>, mut x: usize, mut y: usize) -> bool {
    // this could easily be done using recursive method, but thats kinda boring
    println!("trying from x {}, y {}", x, y);

    // marks already visited nodes x*10+y=index
    let mut visited: Vec<_> = Vec::with_capacity(word.len());
    // let mut visited_stack: Vec<usize> = Vec::new(); // when backtracking to know which nodes to remove from visited
    // visited.resize_with(board.len() * board.get(0).unwrap_or(&Vec::new()).len(), Default::default);

    // marks already visited sides per level (0 - left, 1 - left, top, 2 - left, top, right, 3 -
    // left, top, right, bottom.
    let mut visited_level_sides: Vec<usize> = Vec::new();
    visited_level_sides.resize_with(word.len(), Default::default);


    if board[y][x] != word[0] {
        panic!("at x {}, y {} not actually word 0 {}", x, y, word[0]);
    }

    let mut level = 1;
    visited.push((x, y));

    'word: loop {
        if word.len() == level {
            return true;
        }

        'side: for side in visited_level_sides[level]..4 {
            let (lx, ly) = match side {
                0 if x > 0 => (x-1, y),
                0 => (x, y),
                1 if y > 0 => (x, y-1),
                1 => (x, y),
                2 => (x+1, y),
                3 => (x, y+1),
                _ => break,
            };

            visited_level_sides[level] += 1;

            // handles usize underflow cases
            if lx == x && ly == y {
                continue 'side;
            }

            for (vx, vy) in &visited {
                if vx == &lx && vy == &ly {
                    continue 'side;
                }
            }

            if let Some(yel) = board.get(ly) {
                if let Some(xel) = yel.get(lx) {
                    if xel == &word[level] {
                        // found matching letter, go level up
                        x = lx;
                        y = ly;
                        visited.push((x, y));
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
            x = *last_x;
            y = *last_y;
            continue;
        }

        if level == 1 {
            // all sides and levels exhausted
            break;
        }

        break;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::exist;

    #[test]
    fn it_works() {
        let map = vec![vec!['a', 'b', 'c', 'e'], vec!['s', 'f', 'c', 's'], vec!['a', 'd', 'e', 'e']];
        assert_eq!(true, exist(map.clone(), "abcced".to_string()));
        assert_eq!(true, exist(map.clone(), "see".to_string()));
        assert_eq!(false, exist(map.clone(), "abcb".to_string()));
    }
}
