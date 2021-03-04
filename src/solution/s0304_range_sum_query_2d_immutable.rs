//! [304] Range Sum Query 2D - Immutable
//!
//! Given a 2D matrix matrix, find the sum of the elements inside the rectangle defined by its upper left corner (row1, col1) and lower right corner (row2, col2).
//!
//!
//! <img src="/static/images/courses/range_sum_query_2d.png" border="0" alt="Range Sum Query 2D" /><br />
//! <small>The above rectangle (with the red border) is defined by (row1, col1) = (2, 1) and (row2, col2) = (4, 3), which contains sum = 8.</small>
//!
//!
//! Example:<br>
//!
//! Given matrix = [
//!   [3, 0, 1, 4, 2],
//!   [5, 6, 3, 2, 1],
//!   [1, 2, 0, 1, 5],
//!   [4, 1, 0, 1, 7],
//!   [1, 0, 3, 0, 5]
//! ]
//!
//! sumRegion(2, 1, 4, 3) -> 8
//! sumRegion(1, 1, 2, 2) -> 11
//! sumRegion(1, 2, 2, 4) -> 12
//!
//!
//!
//! Note:<br>
//! <ol>
//! You may assume that the matrix does not change.
//! There are many calls to sumRegion function.
//! You may assume that row1 &le; row2 and col1 &le; col2.
//! </ol>
//!
//!
//! problem: https://leetcode.com/problems/range-sum-query-2d-immutable/
//! discuss: https://leetcode.com/problems/range-sum-query-2d-immutable/discuss/?currentPage=1&orderBy=most_votes&query=

pub struct Solution {}

// submission codes start here

struct NumMatrix {
    row_sums: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        if matrix.len() == 0 {
            return NumMatrix {
                row_sums: vec![vec![]],
            };
        }
        let mut row_sums = vec![vec![0; matrix[0].len() + 1]; matrix.len()];
        for i in 0..matrix.len() {
            let mut sum = 0;
            for j in 0..matrix[i].len() {
                sum += matrix[i][j];
                row_sums[i][j + 1] = sum;
            }
        }
        NumMatrix { row_sums }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        let mut sum = 0;
        for i in row1..=row2 {
            sum += (self.row_sums[i][col2 + 1] - self.row_sums[i][col1]);
        }
        sum
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_304() {}
}
