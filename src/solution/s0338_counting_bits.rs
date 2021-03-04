//! [338] Counting Bits
//!
//! Given a non negative integer number num. For every numbers i in the range 0 &le; i &le; num calculate the number of 1's in their binary representation and return them as an array.
//!
//! Example 1:
//!
//!
//! Input: <span id="example-input-1-1">2</span>
//! Output: <span id="example-output-1">[0,1,1]</span>
//!
//! Example 2:
//!
//!
//! Input: <span id="example-input-1-1">5</span>
//! Output: [0,1,1,2,1,2]
//!
//!
//! Follow up:
//!
//!
//! 	It is very easy to come up with a solution with run time O(n*sizeof(integer)). But can you do it in linear time O(n) /possibly in a single pass?
//! 	Space complexity should be O(n).
//! 	Can you do it like a boss? Do it without using any builtin function like __builtin_popcount in c++ or in any other language.
//!
//!
//! problem: https://leetcode.com/problems/counting-bits/
//! discuss: https://leetcode.com/problems/counting-bits/discuss/?currentPage=1&orderBy=most_votes&query=

pub struct Solution {}

// submission codes start here

impl Solution {
    // pub fn count_bits(num: i32) -> Vec<i32> {
    //     let mut res = vec![];
    //     for num in 0..=num {
    //         res.push(Solution::count_ones(num));
    //     }
    //     res
    // }

    // fn count_ones(mut num: i32) -> i32 {
    //     let mut ones = 0;
    //     while num > 0 {
    //         if num & 1 == 1 {
    //             ones += 1;
    //         }
    //         num >>= 1;
    //     }
    //     ones
    // }

    pub fn count_bits(num: i32) -> Vec<i32> {
        let len = (num + 1) as usize;
        let mut res = vec![0; len];
        for i in 1..=num {
            res[i as usize] = res[i as usize >> 1] + (i & 1);
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_338() {}
}
