pub fn get_longest_palindrome(s: &Vec<char>, max: Vec<char>, i: usize, j: usize) -> Vec<char> {
    let (mut i, mut j) = (i, j);
    let mut current: &[char] = &[];

    while j < s.len() && s[i] == s[j] {
        current = &s[i..j + 1];
        if i == 0 {
            break;
        }
        i -= 1;
        j += 1;
    }
    if current.len() > max.len() {
        return current.to_vec();
    }
    max
}

pub fn longest_palindrome(s: &str) -> String {
    let s = s.chars().collect::<Vec<char>>();
    let mut max: Vec<char> = vec![];

    for i in 0..s.len() {
        max = get_longest_palindrome(&s, max, i, i);
        max = get_longest_palindrome(&s, max, i, i + 1);
    }
    max.into_iter().collect()
}
