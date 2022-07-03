use std::collections::HashMap;

fn decode_message(key: String, message: String) -> String {
    let mut hm: HashMap<char, char> = HashMap::new();
    let mut i = 0;

    for c in key.chars() {
        if c == ' ' {
            continue;
        }
        match hm.get(&c) {
            Some(_) => {}
            None => {
                hm.insert(c, (b'a' + i as u8) as char);
                i += 1;
            }
        }
    }
    hm.insert(' ', ' ');

    message
        .chars()
        .map(|a| hm.get(&a).unwrap())
        .clone()
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_message() {
        assert_eq!(
            decode_message(
                "the quick brown fox jumps over the lazy dog".to_string(),
                "vkbs bs t suepuv".to_string()
            ),
            "this is a secret".to_string()
        );
    }
}
