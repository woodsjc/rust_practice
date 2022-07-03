use std::collections::{HashMap, HashSet};

fn min_deletions(s: String) -> i32 {
    let mut ht: HashMap<char, i32> = HashMap::new();
    let mut tallies: HashSet<i32> = HashSet::new();
    let mut deletes: i32 = 0;

    for c in s.chars() {
        match ht.get_mut(&c) {
            Some(v) => {
                *v += 1;
            }
            None => {
                ht.insert(c, 1);
            }
        }
    }

    for (_, v) in ht.iter() {
        let mut i = *v;
        while let Some(_) = tallies.get(&i) {
            if i <= 0 {
                break;
            }
            deletes += 1;
            i -= 1;
        }
        tallies.insert(i);
    }

    deletes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_deletions() {
        assert_eq!(min_deletions("aab".to_string()), 0);
        assert_eq!(min_deletions("aaabbbcc".to_string()), 2);
        assert_eq!(min_deletions("ceabaacb".to_string()), 2);
    }
}
