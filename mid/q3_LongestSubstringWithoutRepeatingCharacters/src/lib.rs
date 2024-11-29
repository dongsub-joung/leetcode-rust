use std::{cmp, collections::HashMap};

pub fn longest_substring(s: String) -> i32 {
    let mut result= 0;
    let mut left_cursor= 0;
    let mut used: HashMap<char, i32>= HashMap::new();
    let mut used_two: HashMap<char, i32>= HashMap::new();

    let mut chars: Vec<char>= s.chars().collect();

    for (right_cursor, c) in chars.iter().enumerate() {

        
        // if used.contains_key(c) && left_cursor <= *(used.get(c).unwrap()) {
        //     left_cursor= used.get(c).unwrap() +1
        // }else{
        //     let idx= (right_cursor as i32) - left_cursor+ 1;
        //     result= cmp::max(result, idx);
        // }
        // used_two.insert(*c, right_cursor as i32);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = longest_substring("abcabcbb".to_string());
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works2() {
        let result = longest_substring("bbbbb".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn it_works3() {
        let result = longest_substring("pwwkew".to_string());
        assert_eq!(result, 3);
    }
}
