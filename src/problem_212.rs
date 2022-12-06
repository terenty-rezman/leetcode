struct TrieNode {
    letters: [Option<Box<TrieNode>>; 26],
    word_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        const NONE: Option<Box<TrieNode>> = None;
        Self {
            letters: [NONE; 26],
            word_end: false,
        }
    }

    fn add_word(&mut self, word: &str) {
        let mut node = self;
        for &c in word.as_bytes() {
            let idx = c - b'a';
            let n = &mut node.letters[idx as usize];
            if n.is_none() {
                n.replace(Box::new(TrieNode::new()));
            }
            node = n.as_mut().unwrap();
        }
        node.word_end = true;
    }
}

struct Solution;

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = TrieNode::new();

        for w in words {
            trie.add_word(&w);
        }

        let mut result = vec![];

        fn valid_pos(pos: (usize, usize), board: &Vec<Vec<char>>) -> bool {
            if pos.0 >= board.len() {
                return false;
            }
            if pos.1 >= board[0].len() {
                return false;
            }
            true
        }

        fn dfs(
            pos: (usize, usize),
            board: &mut Vec<Vec<char>>,
            word: &mut String,
            trie: &mut Option<Box<TrieNode>>,
            result: &mut Vec<String>,
        ) {
            if trie.is_none() {
                return;
            }

            let curr_char = board[pos.0][pos.1];
            word.push(curr_char);

            let node = trie.as_mut().unwrap();
            if node.word_end {
                result.push(word.to_owned());
                node.word_end = false;
            }

            let old_letter = board[pos.0][pos.1];
            // mark as visited
            board[pos.0][pos.1] = '0';

            for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next_pos = (pos.0 as i32 + i, pos.1 as i32 + j);
                let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

                if valid_pos(next_pos, board) {
                    let next_char = board[next_pos.0][next_pos.1];
                    // if visited
                    if next_char == '0' {
                        continue;
                    }

                    let idx = board[next_pos.0][next_pos.1] as u8 - b'a';
                    let next_node = &mut node.letters[idx as usize];

                    dfs(next_pos, board, word, next_node, result);
                }
            }

            board[pos.0][pos.1] = old_letter;
            word.pop();
        }

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let idx = board[i][j] as u8 - b'a';
                dfs(
                    (i, j),
                    &mut board,
                    &mut "".to_string(),
                    &mut trie.letters[idx as usize],
                    &mut result,
                );
            }
        }

        result
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let board = vec![
            vec!['o', 'a', 'b', 'n'],
            vec!['o', 't', 'a', 'e'],
            vec!['a', 'h', 'k', 'r'],
            vec!['a', 'f', 'l', 'v'],
        ];
        let r = Solution::find_words(
            board,
            vec![
                "oa".to_owned(),
                "oaa".to_owned()
            ],
        );
        dbg!(r);
    }
}
