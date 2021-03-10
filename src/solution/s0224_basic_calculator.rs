//! [224] Basic Calculator
//!
//! Given a string s representing an expression, implement a basic calculator to evaluate it.
//!  
//! Example 1:
//!
//! Input: s = "1 + 1"
//! Output: 2
//!
//! Example 2:
//!
//! Input: s = " 2-1 + 2 "
//! Output: 3
//!
//! Example 3:
//!
//! Input: s = "(1+(4+5+2)-3)+(6+8)"
//! Output: 23
//!
//!  
//! Constraints:
//!
//! 	1 <= s.length <= 3 * 10^5
//! 	s consists of digits, '+', '-', '(', ')', and ' '.
//! 	s represents a valid expression.
//!
//!
//! problem: https://leetcode.com/problems/basic-calculator/
//! discuss: https://leetcode.com/problems/basic-calculator/discuss/?currentPage=1&orderBy=most_votes&query=

pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let (mut res, mut sign, mut num): (i32, i32, i32) = (0, 1, 0);
        let mut stack = Vec::new();
        for c in s.chars() {
            if let Some(n) = c.to_digit(10) {
                num = 10 * num + (n as i32);
            } else if c == '+' || c == '-' {
                res += sign * num;
                num = 0;
                sign = if c == '+' { 1 } else { -1 };
            } else if c == '(' {
                stack.push(res);
                stack.push(sign);
                println!("{}", res);
                res = 0;
                sign = 1;
            } else if c == ')' {
                res += sign * num;
                num = 0;
                res *= stack.pop().unwrap();
                res += stack.pop().unwrap();
            }
        }
        res += sign * num;
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_224() {
        assert_eq!(23, Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()));
    }
}
