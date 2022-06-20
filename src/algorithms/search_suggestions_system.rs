#[derive(Default, Debug)]
struct Trie {
    matches: Vec<String>,
    children: [Option<Box<Trie>>; 27],
}

impl Trie {
    fn new(mut products: Vec<String>) -> Self {
        let mut root = Trie::default();
        products.sort();

        for p in products.iter() {
            let mut node = &mut root;
            for b in p.as_bytes().iter() {
                node = node.children[(b - b'a') as usize].get_or_insert_with(Default::default);
                node.matches.push(p.to_string());
                println!("p:{} added to node with matches:{:?}", p, node.matches);
            }
        }
        root
    }

    fn search(self, search: &str) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        let mut node = &Some(Box::new(self));

        for &b in search.as_bytes().iter() {
            println!("b:{}", b as char);
            match &node.as_ref() {
                Some(n) => {
                    node = &n.children[(b - b'a') as usize];
                    if let Some(o) = node.as_ref() {
                        result.push(
                            o.matches
                                .iter()
                                .take(3)
                                .map(|s| String::from(s))
                                .collect::<Vec<String>>(),
                        )
                    } else {
                        result.push(vec![]);
                    }
                }
                None => result.push(vec![]),
            }
        }

        result
    }
}

fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    let trie = Trie::new(products);
    trie.search(&search_word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggested_products() {
        assert_eq!(
            suggested_products(
                vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect::<Vec<String>>(),
                "mouse".to_string()
            ),
            vec![
                vec!["mobile", "moneypot", "monitor"],
                vec!["mobile", "moneypot", "monitor"],
                vec!["mouse", "mousepad"],
                vec!["mouse", "mousepad"],
                vec!["mouse", "mousepad"]
            ]
        );
        assert_eq!(
            suggested_products(
                vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect::<Vec<String>>(),
                "mxxd".to_string()
            ),
            vec![vec!["mobile", "moneypot", "monitor"], vec![], vec![], vec![]]
        );

        let none: Vec<Vec<String>> = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
        assert_eq!(
            suggested_products(
                vec!["havana"]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect::<Vec<String>>(),
                "tatiana".to_string()
            ),
            none
        );
    }
}
