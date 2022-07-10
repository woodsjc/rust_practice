fn is_interleaving(s1: String, s2: String, s3: String) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }
    let s1 = s1.chars().collect::<Vec<char>>();
    let s2 = s2.chars().collect::<Vec<char>>();
    let s3 = s3.chars().collect::<Vec<char>>();
    let mut dp: Vec<Vec<bool>> = vec![vec![false; s2.len() + 1]; s1.len() + 1];

    for i in 0..=s1.len() {
        for j in 0..=s2.len() {
            dp[i][j] = match (i, j) {
                (0, 0) => true,
                (0, _) => dp[i][j - 1] && s2[j - 1] == s3[j - 1],
                (_, 0) => dp[i - 1][j] && s1[i - 1] == s3[i - 1],
                _ => {
                    let k = i + j;
                    (dp[i - 1][j] && s1[i - 1] == s3[k]) || (dp[i][j - 1] && s2[j - 1] == s3[k])
                }
            }
        }
    }
    dp[s1.len()][s2.len()]
}

fn is_interleaving_dfs(s1: String, s2: String, s3: String) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }
    let s1 = s1.chars().collect::<Vec<char>>();
    let s2 = s2.chars().collect::<Vec<char>>();
    let s3 = s3.chars().collect::<Vec<char>>();
    use std::cell::RefCell;
    use std::rc::Rc;
    let dp: Rc<RefCell<Vec<Vec<Option<bool>>>>> =
        Rc::new(RefCell::new(vec![vec![None; s2.len()]; s1.len()]));

    fn dfs(
        s1: &Vec<char>,
        s2: &Vec<char>,
        s3: &Vec<char>,
        mut i: usize,
        mut j: usize,
        dp: Rc<RefCell<Vec<Vec<Option<bool>>>>>,
    ) -> bool {
        while i + j < s3.len() {
            println!(
                "i:{}, j:{}, k:{}, s3.len():{}, dp.len():{}, dp[0].len():{}",
                i,
                j,
                i + j,
                s3.len(),
                dp.borrow().len(),
                dp.borrow()[0].len()
            );
            if i >= s1.len() && j >= s2.len() {
                return false;
            } else if i >= s1.len() {
                return s2[j..] == s3[i + j..];
            } else if j >= s2.len() {
                return s1[i..] == s3[i + j..];
            } else if let Some(d) = dp.borrow()[i][j] {
                return d;
            }

            //can't have same if block with above let borrow scope
            if s1[i] == s3[i + j] && s2[j] == s3[i + j] {
                dp.borrow_mut()[i][j] = Some(
                    dfs(s1, s2, s3, i + 1, j, Rc::clone(&dp))
                        || dfs(s1, s2, s3, i, j + 1, Rc::clone(&dp)),
                );
                return dp.borrow()[i][j].unwrap();
            } else if s1[i] == s3[i + j] {
                i += 1;
            } else if s2[j] == s3[i + j] {
                j += 1;
            } else {
                dp.borrow_mut()[i][j] = Some(false);
                return false;
            }
        }
        if i < s1.len() && j < s2.len() {
            dp.borrow_mut()[i][j] = Some(true);
        }
        true
    }

    dfs(&s1, &s2, &s3, 0, 0, dp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_interleaving() {
        assert_eq!(
            is_interleaving(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbbaccc".to_string()
            ),
            false
        );
        assert_eq!(
            is_interleaving(
                "abababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string(),
                "babababababababababababababababababababababababababababababababababababababababababababababababaaaba".to_string(),
                "abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string(),
            ),
            false
        );
    }

    #[test]
    fn test_is_interleaving_dfs() {
        assert_eq!(
            is_interleaving_dfs(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbbaccc".to_string()
            ),
            false
        );
        assert_eq!(
            is_interleaving_dfs(
                "abababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string(),
                "babababababababababababababababababababababababababababababababababababababababababababababababaaaba".to_string(),
                "abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string(),
            ),
            false
        );
    }
}
