//! [303] Range Sum Query - Immutable
//!
//! Given an integer array nums, find the sum of the elements between indices i and j (i &le; j), inclusive.
//! Implement the NumArray class:
//!
//! 	NumArray(int[] nums) Initializes the object with the integer array nums.
//! 	int sumRange(int i, int j) Return the sum of the elements of the nums array in the range [i, j] inclusive (i.e., sum(nums[i], nums[i + 1], ... , nums[j]))
//!
//!  
//! Example 1:
//!
//! Input
//! ["NumArray", "sumRange", "sumRange", "sumRange"]
//! [[[-2, 0, 3, -5, 2, -1]], [0, 2], [2, 5], [0, 5]]
//! Output
//! [null, 1, -1, -3]
//! Explanation
//! NumArray numArray = new NumArray([-2, 0, 3, -5, 2, -1]);
//! numArray.sumRange(0, 2); // return 1 ((-2) + 0 + 3)
//! numArray.sumRange(2, 5); // return -1 (3 + (-5) + 2 + (-1))
//! numArray.sumRange(0, 5); // return -3 ((-2) + 0 + 3 + (-5) + 2 + (-1))
//!
//!  
//! Constraints:
//!
//! 	0 <= nums.length <= 10^4
//! 	-10^5 <= nums[i] <= 10^5
//! 	0 <= i <= j < nums.length
//! 	At most 10^4 calls will be made to sumRange.
//!
//!
//! problem: https://leetcode.com/problems/range-sum-query-immutable/
//! discuss: https://leetcode.com/problems/range-sum-query-immutable/discuss/?currentPage=1&orderBy=most_votes&query=

pub struct Solution {}

// submission codes start here

struct NumArray {
    sums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sum = 0;
        let mut sums = vec![0];
        for v in nums.iter() {
            sum += v;
            sums.push(sum);
        }
        NumArray { sums }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let (i, j) = (i as usize, j as usize);
        self.sums[j + 1] - self.sums[i]
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(i, j);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_303() {}
}
