//! [232] Implement Queue using Stacks
//!
//! Implement a first in first out (FIFO) queue using only two stacks. The implemented queue should support all the functions of a normal queue (push, peek, pop, and empty).
//! Implement the MyQueue class:
//!
//! 	void push(int x) Pushes element x to the back of the queue.
//! 	int pop() Removes the element from the front of the queue and returns it.
//! 	int peek() Returns the element at the front of the queue.
//! 	boolean empty() Returns true if the queue is empty, false otherwise.
//!
//! Notes:
//!
//! 	You must use only standard operations of a stack, which means only push to top, peek/pop from top, size, and is empty operations are valid.
//! 	Depending on your language, the stack may not be supported natively. You may simulate a stack using a list or deque (double-ended queue) as long as you use only a stack's standard operations.
//!
//! Follow-up: Can you implement the queue such that each operation is <a href="https://en.wikipedia.org/wiki/Amortized_analysis" target="_blank">amortized</a> O(1) time complexity? In other words, performing n operations will take overall O(n) time even if one of those operations may take longer.
//!  
//! Example 1:
//!
//! Input
//! ["MyQueue", "push", "push", "peek", "pop", "empty"]
//! [[], [1], [2], [], [], []]
//! Output
//! [null, null, null, 1, 1, false]
//! Explanation
//! MyQueue myQueue = new MyQueue();
//! myQueue.push(1); // queue is: [1]
//! myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
//! myQueue.peek(); // return 1
//! myQueue.pop(); // return 1, queue is [2]
//! myQueue.empty(); // return false
//!
//!  
//! Constraints:
//!
//! 	1 <= x <= 9
//! 	At most 100 calls will be made to push, pop, peek, and empty.
//! 	All the calls to pop and peek are valid.
//!
//!
//! problem: https://leetcode.com/problems/implement-queue-using-stacks/
//! discuss: https://leetcode.com/problems/implement-queue-using-stacks/discuss/?currentPage=1&orderBy=most_votes&query=

pub struct Solution {}

// submission codes start here

struct MyQueue {
    head: Vec<i32>,
    tail: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyQueue {
            head: Vec::new(),
            tail: Vec::new(),
        }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.tail.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.head.is_empty() {
            while let Some(v) = self.tail.pop() {
                self.head.push(v);
            }
        }
        self.head.pop().unwrap()
    }

    /** Get the front element. */
    fn peek(&self) -> i32 {
        if self.head.is_empty() {
            *self.tail.first().unwrap()
        } else {
            *self.head.last().unwrap()
        }
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.head.is_empty() && self.tail.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_232() {}
}
