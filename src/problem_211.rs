struct WordDictionary {
    data: Option<Box<TrieNode>>,
}

struct TrieNode {
    letters: [Option<Box<TrieNode>>; 26],
    word_ends_here: bool,
}

impl TrieNode {
    fn new() -> Self {
        const NONE: Option<Box<TrieNode>> = None;
        Self {
            letters: [NONE; 26],
            word_ends_here: false,
        }
    }
}

impl WordDictionary {
    fn new() -> Self {
        Self { data: None }
    }

    fn add_word(&mut self, word: String) {
        if word.is_empty() {
            return;
        }

        if self.data.is_none() {
            self.data.replace(Box::new(TrieNode::new()));
        }
        let mut cur_node = self.data.as_mut().unwrap();

        let word = word.as_bytes();
        for &l in word {
            let idx = l - b'a';
            let n = &mut cur_node.letters[idx as usize];
            if n.is_none() {
                n.replace(Box::new(TrieNode::new()));
            }
            cur_node = n.as_mut().unwrap();
        }

        cur_node.word_ends_here = true;
    }

    fn search(&self, word: String) -> bool {
        if word.is_empty() {
            return true;
        }

        if self.data.is_none() {
            return false;
        }

        fn dfs(word: &[u8], node: &TrieNode) -> bool {
            if word.is_empty() {
                return node.word_ends_here;
            }

            let l = word[0];
            if l == b'.' {
                for n in node.letters.iter().flatten() {
                    if dfs(&word[1..], n) {
                        return true;
                    }
                }

                return false;
            }

            let idx = l - b'a';
            if let Some(ref n) = node.letters[idx as usize] {
                return dfs(&word[1..], n);
            }

            false
        }

        dfs(word.as_bytes(), self.data.as_ref().unwrap())
    }
}

pub mod tests {
    use super::WordDictionary;

    pub fn test() {
        let mut dict = WordDictionary::new();
        dict.add_word("bad".into());
        dict.add_word("dad".into());
        dict.add_word("mad".into());
        dbg!(dict.search("pad".into()));
        dbg!(dict.search("bad".into()));
        dbg!(dict.search(".ad".into()));
        dbg!(dict.search("b..".into()));
    }
}
