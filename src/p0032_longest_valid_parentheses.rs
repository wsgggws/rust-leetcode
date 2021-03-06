// 32. Longest Valid Parentheses
// Hard

// Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.

// Example 1:

// Input: "(()"
// Output: 2
// Explanation: The longest valid parentheses substring is "()"
// Example 2:

// Input: ")()())"
// Output: 4
// Explanation: The longest valid parentheses substring is "()()"

pub struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![];
        let mut count = 0;
        let mut result = 0;

        for parenthes in s.chars() {
            if parenthes == '(' {
                stack.push(count);
                count = 0;
            } else if !stack.is_empty() && parenthes == ')' {
                if let Some(value) = stack.pop() {
                    count += 1 + value;
                }
                result = i32::max(result, count)
            } else {
                count = 0;
            }
        }
        result * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_valid_parentheses_test() {
        assert_eq!(Solution::longest_valid_parentheses(")()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses(")()()".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("())()()".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("(()())(()".to_string()), 6);
    }
}
