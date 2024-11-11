use std::collections::HashMap;

struct Solution{}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0  || x == 0{
            return false;
        }
        let str_v: Vec<char>= x.to_string().chars()
            .into_iter()
            .map(|f| f)
            .collect();

        let mut buf: HashMap<char, i32>= HashMap::new();
        
        let size= str_v.len();
        for e in str_v{
            buf.insert(e, 1);
            if buf.contains_key(&e) {
                buf.insert(e, 0);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_palindrome(121);
        assert_eq!(result, true);
    }

    #[test]
    fn it_works2() {
        let result = Solution::is_palindrome(10);
        assert_eq!(result, false);
    }
}
