use std::collections::HashMap;

fn is_anagram(s: String, t: String) -> bool {
    let mut hs: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        if let Some(r) = hs.get_mut(&c) {
            *r += 1;
        } else {
            hs.insert(c, 1);
        }
    }
    for c in t.chars() {
        if let Some(r) = hs.get_mut(&c) {
            *r -= 1;
        } else {
            return false;
        }
    }
    for (_, v) in hs {
        if v != 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(is_anagram("anagram".to_string(), "nagaram".to_string()), true);
    }
}
