struct Trie {
    data: TrieNode,
}

struct TrieNode {
    letters: [Option<Box<TrieNode>>; 26],
    word_ends_here: bool,
}

impl TrieNode {
    fn new() -> Self {
        const EMPTY_LETTER: Option<Box<TrieNode>> = None;
        Self {
            letters: [EMPTY_LETTER; 26],
            word_ends_here: false,
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie {
            data: TrieNode::new()
        }
    }

    fn insert(&mut self, word: String) {
        if word.is_empty() {
            return;
        }

        let s = word.as_bytes();
        let mut node = &mut self.data;
        for &l in s {
            let idx = l - b'a';
            let n = &mut node.letters[idx as usize];
            if n.is_none() {
                n.replace(Box::new(TrieNode::new()));
            }
            node = n.as_mut().unwrap();
        }

        node.word_ends_here = true;
    }

    fn search(&self, word: String) -> bool {
        if word.is_empty() {
            return true;
        }

        let last = Self::last_letter_node(self, word);

        if let Some(l) = last {
            return l.word_ends_here;
        }

        false
    }

    fn starts_with(&self, prefix: String) -> bool { 
        if prefix.is_empty() {
            return true;
        }

        let last = Self::last_letter_node(self, prefix);

        if last.is_some() {
            return true;
        }

        false
    }

    fn last_letter_node(&self, word: String) -> Option<&TrieNode> {
        let s = word.as_bytes();
        let mut node = &self.data;
        for &l in s {
            let idx = l - b'a';
            let n = &node.letters[idx as usize];
            if n.is_none() {
                return None;
            }
            node = n.as_ref().unwrap();
        }

        Some(node)
    }
}

pub mod tests {
    use super::Trie;

    pub fn test() {
        let mut trie = Trie::new();
        let word = "apple".to_string();

        dbg!(trie.search(word.clone()));
        dbg!(trie.insert(word.clone()));
        dbg!(trie.search(word));
        dbg!(trie.search("app".to_string()));
        dbg!(trie.starts_with("app".to_string()));
        dbg!(trie.insert("app".to_string()));
        dbg!(trie.search("app".to_string()));
    }
}
