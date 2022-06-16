fn min_distance(word1: String, word2: String) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![i32::MAX; word2.len()]; word1.len()];
    let word1 = word1.chars().collect::<Vec<char>>();
    let word2 = word2.chars().collect::<Vec<char>>();

    fn solve(w1: &Vec<char>, w2: &Vec<char>, i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if i == w1.len() && j == w2.len() {
            return 0;
        } else if i == w1.len() || j == w2.len() {
            return i32::max(w1.len() as i32 - i as i32, w2.len() as i32 - j as i32);
        } else if dp[i][j] != i32::MAX {
            return dp[i][j];
        } else if w1[i] == w2[j] {
            return solve(w1, w2, i + 1, j + 1, dp);
        } else {
            dp[i][j] = 1 + i32::min(solve(w1, w2, i + 1, j, dp), solve(w1, w2, i, j + 1, dp));
            return dp[i][j];
        }
    }

    solve(&word1, &word2, 0, 0, &mut dp)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_distance() {
        assert_eq!(min_distance(String::from("sea"), String::from("eat")), 2);
        assert_eq!(
            min_distance(String::from("leetcode"), String::from("etco")),
            4
        );
        assert_eq!(min_distance(String::from("tsea"), String::from("eat")), 3);
        assert_eq!(
            min_distance(
                String::from("dinitrophenylhydrazine"),
                String::from("acetylphenylhydrazine")
            ),
            11
        );
    }
}
