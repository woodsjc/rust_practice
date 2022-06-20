#[derive(Default)]
struct Trie {
    i: i32,
    children: [Option<Box<Trie>>; 27],
}

struct WordFilter {
    trie: Trie,
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::default();
        for (i, word) in words.iter().enumerate() {
            let s = format!("{}{{{}", &word, &word);

            println!("s:{}", s);
            for j in 0..word.len() {
                let mut node = &mut trie;
                for &b in &s.as_bytes()[j..] {
                    println!(
                        "b:{}, &b:{}, &s.as_bytes()[j..]:{:?}",
                        b,
                        &b,
                        &s.as_bytes()[j..]
                    );
                    node = node.children[(b - b'a') as usize].get_or_insert_with(Default::default);
                    node.i = i as i32;
                }
            }
        }
        Self { trie }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        let mut node = &self.trie;
        let s = format!("{}{{{}", &suffix, &prefix);
        println!("s:{}", s);
        for &b in s.as_bytes() {
            if let Some(n) = &node.children[(b - b'a') as usize] {
                node = n.as_ref();
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
