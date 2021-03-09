//! [1047] Remove All Adjacent Duplicates In String
//!
//! Given a string S of lowercase letters, a duplicate removal consists of choosing two adjacent and equal letters, and removing them.
//!
//! We repeatedly make duplicate removals on S until we no longer can.
//!
//! Return the final string after all such duplicate removals have been made.  It is guaranteed the answer is unique.
//!
//!  
//!
//! Example 1:
//!
//!
//! Input: <span id="example-input-1-1">"abbaca"</span>
//! Output: <span id="example-output-1">"ca"</span>
//! Explanation:
//! For example, in "abbaca" we could remove "bb" since the letters are adjacent and equal, and this is the only possible move.  The result of this move is that the string is "aaca", of which only "aa" is possible, so the final string is "ca".
//!
//!
//!  
//!
//! Note:
//!
//! <ol>
//! 	1 <= S.length <= 20000
//! 	S consists only of English lowercase letters.
//! </ol>
//!
//!
//! problem: https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/
//! discuss: https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/discuss/?currentPage=1&orderBy=most_votes&query=

pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();
        for c in s.chars() {
            if let Some(top) = stack.last() {
                if *top == c {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            } else {
                stack.push(c);
            }
        }
        stack.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1047() {}
}
