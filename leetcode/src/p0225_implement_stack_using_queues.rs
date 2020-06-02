// 225. Implement Stack using Queues
// Easy
// Implement the following operations of a stack using queues.

// push(x) -- Push element x onto stack.
// pop() -- Removes the element on top of the stack.
// top() -- Get the top element.
// empty() -- Return whether the stack is empty.
// Example:

// MyStack stack = new MyStack();

// stack.push(1);
// stack.push(2);
// stack.top();   // returns 2
// stack.pop();   // returns 2
// stack.empty(); // returns false
// Notes:

// You must use only standard operations of a queue -- which means only push to back, peek/pop from front, size, and is empty operations are valid.
// Depending on your language, queue may not be supported natively. You may simulate a queue by using a list or deque (double-ended queue), as long as you use only standard operations of a queue.
// You may assume that all operations are valid (for example, no pop or top operations will be called on an empty stack).



/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
* let ret_4: bool = obj.empty();
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */


use std::collections::VecDeque;

struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
}

#[allow(dead_code)]
impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
        }
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        if self.q1.is_empty() {
            self.q1.push_back(x);
            while let Some(v) = self.q2.pop_front() {
                self.q1.push_back(v);
            }
        } else {
            self.q2.push_back(x);
            while let Some(v) = self.q1.pop_front() {
                self.q2.push_back(v);
            }
        }
    }

    fn pop(&mut self) -> i32 {
        if self.q1.is_empty() {
            self.q2.pop_front().unwrap()
        } else {
            self.q1.pop_front().unwrap()
        }
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        if self.q1.is_empty() {
            *self.q2.front().unwrap_or(&0)
        } else {
            *self.q1.front().unwrap_or(&0)
        }
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_stack_test() {
        let mut obj = MyStack::new();
        obj.push(2);
        let ret_2: i32 = obj.pop();
        assert_eq!(ret_2, 2);
        let ret_3: i32 = obj.top();
        println!("ret_3: {}", ret_3);
        let ret_4: bool = obj.empty();
        println!("ret_4: {}", ret_4);
    }
}
