fn length_of_longest_substring(s: String) -> i32 {
    let mut prior: Vec<u32> = vec![];
    let mut max = 0;

    for c in s.chars().into_iter() {
        let i = prior.iter().position(|&r| r == c as u32);
        match i {
            Some(i) => prior = prior.drain(i + 1..).collect(),
            None => (),
        }
        prior.push(c as u32);
        max = i32::max(max, prior.len() as i32);
        print!("i:{:?}, prior:{:?}\n", i, prior);
    }
    max
}

fn length_of_longest_substring_with_hashtable(s: String) -> i32 {
    use std::collections::HashMap;
    let mut prior: HashMap<char, usize> = HashMap::new();
    let mut max = 0;
    let mut left = 0;

    for (right, c) in s.chars().into_iter().enumerate() {
        if prior.contains_key(&c) {
            left = usize::max(prior[&c] + 1, left);
        }
        prior.insert(c, right);
        max = i32::max(max, (right - left + 1) as i32);
        print!(
            "left:{:?}, right:{:?}, prior:{:?}, max:{}\n",
            left, right, prior, max
        );
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
        assert_eq!(length_of_longest_substring(String::from("dvdf")), 3);
    }

    #[test]
    fn test_length_of_longest_substring_with_hashtable() {
        assert_eq!(
            length_of_longest_substring_with_hashtable(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            length_of_longest_substring_with_hashtable(String::from("dvdf")),
            3
        );
        assert_eq!(
            length_of_longest_substring_with_hashtable(String::from("abba")),
            2
        );
    }
}
