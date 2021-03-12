//! [227] Basic Calculator II
//!
//! Given a string s which represents an expression, evaluate this expression and return its value.
//! The integer division should truncate toward zero.
//!  
//! Example 1:
//! Input: s = "3+2*2"
//! Output: 7
//! Example 2:
//! Input: s = " 3/2 "
//! Output: 1
//! Example 3:
//! Input: s = " 3+5 / 2 "
//! Output: 5
//!  
//! Constraints:
//!
//! 	1 <= s.length <= 3 * 10^5
//! 	s consists of integers and operators ('+', '-', '*', '/') separated by some number of spaces.
//! 	s represents a valid expression.
//! 	All the integers in the expression are non-negative integers in the range [0, 2^31 - 1].
//! 	The answer is guaranteed to fit in a 32-bit integer.
//!
//!
//! problem: https://leetcode.com/problems/basic-calculator-ii/
//! discuss: https://leetcode.com/problems/basic-calculator-ii/discuss/?currentPage=1&orderBy=most_votes&query=

pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut last_op = b'+';
        let mut num = 0;
        for (i, b) in s.bytes().enumerate() {
            let is_digit = b.is_ascii_digit();
            if is_digit {
                num = num * 10 + (b - b'0') as i32;
            }
            if (!is_digit && b != b' ') || i == s.len() - 1 {
                match last_op {
                    b'+' => stack.push(num),
                    b'-' => stack.push(-num),
                    b'*' => *stack.last_mut().unwrap() *= num,
                    b'/' => *stack.last_mut().unwrap() /= num,
                    _ => unreachable!(),
                }
                last_op = b;
                num = 0;
            }
        }
        stack.iter().sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_227() {}
}
