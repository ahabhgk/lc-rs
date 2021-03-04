//! [354] Russian Doll Envelopes
//!
//! You are given a 2D array of integers envelopes where envelopes[i] = [wi, hi] represents the width and the height of an envelope.
//! One envelope can fit into another if and only if both the width and height of one envelope is greater than the width and height of the other envelope.
//! Return the maximum number of envelopes can you Russian doll (i.e., put one inside the other).
//! Note: You cannot rotate an envelope.
//!  
//! Example 1:
//!
//! Input: envelopes = [[5,4],[6,4],[6,7],[2,3]]
//! Output: 3
//! Explanation: The maximum number of envelopes you can Russian doll is 3 ([2,3] => [5,4] => [6,7]).
//!
//! Example 2:
//!
//! Input: envelopes = [[1,1],[1,1],[1,1]]
//! Output: 1
//!
//!  
//! Constraints:
//!
//! 	1 <= envelopes.length <= 5000
//! 	envelopes[i].length == 2
//! 	1 <= wi, hi <= 10^4
//!
//!
//! problem: https://leetcode.com/problems/russian-doll-envelopes/
//! discuss: https://leetcode.com/problems/russian-doll-envelopes/discuss/?currentPage=1&orderBy=most_votes&query=

use std::cmp::Ordering;

pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        if envelopes.len() == 0 {
            return 0;
        }
        let mut envelopes = envelopes;
        // eq to envelopes.sort();
        envelopes.sort_unstable_by(|a, b| {
            if a[0].cmp(&b[0]) == Ordering::Equal {
                a[1].cmp(&b[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        let mut dp = vec![1; envelopes.len()];
        for i in 1..envelopes.len() {
            for j in 0..i {
                if envelopes[i][0] > envelopes[j][0] && envelopes[i][1] > envelopes[j][1] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        *dp.iter().max().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_354() {}
}
