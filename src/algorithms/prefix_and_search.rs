#[derive(Default)]
struct Trie {
    i: i32,
    children: std::collections::HashMap<char, Trie>,
}

struct WordFilter {
    trie: Trie,
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::default();
        for (i, word) in words.iter().enumerate() {
            let s = format!("{}#{}", &word, &word);

            println!("s:{}", s);
            for (j, _) in word.chars().enumerate() {
                //for j in 0..word.len() {
                let mut node = &mut trie;
                for c in s.chars().skip(j) {
                    println!("c:{}, s:{}, j:{}", c, s, j);
                    node = node.children.entry(c).or_insert(Trie::default());
                    node.i = i as i32;
                }
            }
        }
        Self { trie }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        let mut node = &self.trie;
        let s = format!("{}#{}", &suffix, &prefix);
        println!("s:{}", s);
        for c in s.chars() {
            if let Some(n) = &node.children.get(&c) {
                node = n;
            } else {
                return -1;
            }
        }
        node.i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_wordfilter() {
        let words = vec!["apple".to_string()];
        let obj = WordFilter::new(words);
        let ret_1 = obj.f("a".to_string(), "e".to_string());
        assert_eq!(ret_1, 0);
    }
}
