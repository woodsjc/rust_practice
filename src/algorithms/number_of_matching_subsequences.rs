use std::collections::HashMap;

fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    let mut hm: HashMap<char, Vec<&str>> = HashMap::new();
    let mut count = 0;

    for w in words.iter() {
        let first = w.chars().next().unwrap();
        match hm.get_mut(&first) {
            Some(r) => {
                r.push(&w[1..]);
            }
            None => {
                hm.insert(first, vec![&w[1..]]);
            }
        }
    }

    for c in s.chars() {
        if let Some(hit) = hm.remove(&c) {
            for h in hit.iter() {
                if let Some(first) = h.chars().next() {
                    match hm.get_mut(&first) {
                        Some(r) => {
                            r.push(&h[1..]);
                        }
                        None => {
                            hm.insert(first, vec![&h[1..]]);
                        }
                    }
                } else {
                    count += 1;
                }
            }
        }
        println!("c:{}, hm:{:?}", c, hm);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_matching_subseq() {
        assert_eq!(
            num_matching_subseq(
                "dsahjpjauf".to_string(),
                vec![
                    "ahjpjau".to_string(),
                    "ja".to_string(),
                    "ahbwzgqnuk".to_string(),
                    "tnmlanowax".to_string()
                ]
            ),
            2
        );
        assert_eq!(
            num_matching_subseq(
                "abcde".to_string(),
                vec![
                    "a".to_string(),
                    "bb".to_string(),
                    "acd".to_string(),
                    "ace".to_string()
                ]
            ),
            3
        );
    }
}
