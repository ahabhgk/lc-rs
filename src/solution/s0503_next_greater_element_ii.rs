//! [503] Next Greater Element II
//!
//!
//! Given a circular array (the next element of the last element is the first element of the array), print the Next Greater Number for every element. The Next Greater Number of a number x is the first greater number to its traversing-order next in the array, which means you could search circularly to find its next greater number. If it doesn't exist, output -1 for this number.
//!
//!
//! Example 1:<br />
//!
//! Input: [1,2,1]
//! Output: [2,-1,2]
//! Explanation: The first 1's next greater number is 2; </br>The number 2 can't find next greater number; </br>The second 1's next greater number needs to search circularly, which is also 2.
//!
//!
//!
//! Note:
//! The length of given array won't exceed 10000.
//!
//!
//! problem: https://leetcode.com/problems/next-greater-element-ii/
//! discuss: https://leetcode.com/problems/next-greater-element-ii/discuss/?currentPage=1&orderBy=most_votes&query=

pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![-1; nums.len()];
        let mut stack = Vec::new();
        let len = nums.len();
        for i in 0..2 * len {
            while !stack.is_empty() && nums[*stack.last().unwrap()] < nums[i % len] {
                res[stack.pop().unwrap()] = nums[i % len];
            }
            stack.push(i % len);
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_503() {}
}
