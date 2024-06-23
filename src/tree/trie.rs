//! 前缀树

pub struct Trie {
    pass: usize,
    end: usize,
    nexts: Vec<Option<Box<Trie>>>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            pass: 0,
            end: 0,
            nexts: vec![None; 26],
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        node.pass += 1;
        for &c in word.as_bytes() {
            let i = (c - b'a') as usize;
            if node.nexts[i].is_none() {
                node.nexts[i] = Some(Box::new(Trie::new()));
                node.nexts[i].as_mut().unwrap().pass += 1;
                node = node.nexts[i].as_mut().unwrap();
            }
        }
        node.end += 1;
    }

    pub fn count_words_equal_to(&self, word: &str) -> usize {
        let mut node = self;
        for &c in word.as_bytes() {
            let i = (c - b'a') as usize;
            if node.nexts[i].is_none() {
                return 0;
            }
            node = node.nexts[i].as_ref().unwrap();
        }
        node.end as usize
    }

    pub fn search(&self, word: &str) -> bool {
        let mut node = self;
        for &c in word.as_bytes() {
            let i = (c - b'a') as usize;
            if node.nexts[i].is_none() {
                return false;
            }
            node = node.nexts[i].as_ref().unwrap();
        }
        node.end > 0
    }

    pub fn exist_words_starting_with(&self, prefix: &str) -> bool {
        let mut node = self;
        for &c in prefix.as_bytes() {
            let i = (c - b'a') as usize;
            if node.nexts[i].is_none() {
                return false;
            }
            node = node.nexts[i].as_ref().unwrap();
        }
        true
    }

    pub fn count_words_starting_with(&self, prefix: &str) -> usize {
        let mut node = self;
        for &c in prefix.as_bytes() {
            let i = (c - b'a') as usize;
            if node.nexts[i].is_none() {
                return 0;
            }
            node = node.nexts[i].as_ref().unwrap();
        }
        node.pass as usize
    }

    pub fn erase(&mut self, word: &str) {
        let count = self.count_words_equal_to(&word);
        if count == 0 {
            return;
        }

        let mut node = self;
        node.pass -= 1;
        for &c in word.as_bytes() {
            let i = (c - b'a') as usize;
            if node.nexts[i].as_ref().unwrap().pass - 1 == 0 {
                let next_node = node.nexts[i].as_mut().unwrap();
                node.nexts[i] = None;
                free(next_node);
                return;
            } else {
                node.nexts[i].as_mut().unwrap().pass -= 1;
            }
            node = node.nexts[i].as_mut().unwrap();
        }
    }

    pub fn reset(&mut self, word: &str) {
        let count = self.count_words_equal_to(&word);
        if count == 0 {
            return;
        }

        let mut node = self;
        node.pass -= count;
        for &c in word.as_bytes() {
            let i = (c - b'a') as usize;
            if node.nexts[i].as_ref().unwrap().pass - count == 0 {
                let next_node = node.nexts[i].as_mut().unwrap();
                node.nexts[i] = None;
                free(next_node);
                return;
            } else {
                node.nexts[i].as_mut().unwrap().pass -= count;
            }
            node = node.nexts[i].as_mut().unwrap();
        }
    }
}
