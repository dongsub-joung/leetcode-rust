// https://leetcode.com/problems/valid-palindrome/description/

use std::{collections::VecDeque};

pub fn is_palindrome(s: String) -> bool {
    let v: Vec<_>= s.trim().chars().map(|f| f.to_ascii_lowercase()).collect();
    let mut deque: VecDeque<_> = VecDeque::from(v);

    println!("{:?}", deque);

    while deque.len() > 1 {
        if deque.pop_front().unwrap() != deque.pop_back().unwrap() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // ['a', ' ', 'm', 'a', 'n', ',', ' ', 'a', ' ', 'p', 'l', 'a', 'n', ',', ' ', 'a', ' ', 'c', 'a', 'n', 'a', 'l', ':', ' ', 'p', 'a', 'n', 'a', 'm', 'a']    #[test]
    #[test]
    fn it_works() {
        let result = is_palindrome("A man, a plan, a canal: Panama".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn it_works2() {
        let result = is_palindrome("race a car".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn it_works3() {
        let result = is_palindrome(" ".to_string());
        assert_eq!(result, true);
    }
}
