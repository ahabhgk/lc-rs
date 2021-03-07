//! [131] Palindrome Partitioning
//!
//! Given a string s, partition s such that every substring of the partition is a palindrome. Return all possible palindrome partitioning of s.
//! A palindrome string is a string that reads the same backward as forward.
//!  
//! Example 1:
//! Input: s = "aab"
//! Output: [["a","a","b"],["aa","b"]]
//! Example 2:
//! Input: s = "a"
//! Output: [["a"]]
//!  
//! Constraints:
//!
//! 	1 <= s.length <= 16
//! 	s contains only lowercase English letters.
//!
//!
//! problem: https://leetcode.com/problems/palindrome-partitioning/
//! discuss: https://leetcode.com/problems/palindrome-partitioning/discuss/?currentPage=1&orderBy=most_votes&query=

pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn is_palindrome(s: &[u8], mut i: usize, mut j: usize) -> bool {
            while i < j {
                if s[i] != s[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        };
        fn dfs(i: usize, s: &str, ans: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
            let len = s.len();
            if i == len {
                res.push(ans.to_vec());
                return;
            }
            for j in i..len {
                if is_palindrome(s.as_bytes(), i, j) {
                    ans.push(s[i..j + 1].to_string());
                    dfs(j + 1, s, ans, res);
                    ans.pop();
                }
            }
        };

        let mut res = Vec::new();
        dfs(0, &s, &mut Vec::new(), &mut res);
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_131() {}
}
