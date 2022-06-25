fn minimun_length_encoding(words: Vec<String>) -> i32 {
    //check if any words are suffix of another
    let mut remaining = words.clone();
    remaining.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

    let mut i: i32 = 0;
    while (i as usize) < remaining.len() {
        let w = &remaining[i as usize];
        for base in remaining[i as usize + 1..].iter() {
            if base.ends_with(w) {
                remaining.remove(i as usize);
                i -= 1;
                break;
            }
        }
        i += 1;
    }

    println!("remaining:{:?}, joined:{}", remaining, remaining.join("#"));
    remaining.join("#").len() as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimun_length_encoding() {
        assert_eq!(
            minimun_length_encoding(vec![
                "time".to_string(),
                "me".to_string(),
                "bell".to_string()
            ]),
            10
        );
    }
}
