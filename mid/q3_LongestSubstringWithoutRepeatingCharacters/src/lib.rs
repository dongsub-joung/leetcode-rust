use std::collections::HashSet;


// https://github.com/doocs/leetcode/blob/main/solution/0000-0099/0003.Longest%20Substring%20Without%20Repeating%20Characters/README_EN.md
pub fn longest_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let mut ss = HashSet::new();
    let mut i = 0;
    s.iter()
        .map(|c| {
            while ss.contains(&c) {
                ss.remove(&s[i]);
                i += 1;
            }
            ss.insert(c);
            ss.len()
        })
        .max()
        .unwrap_or(0) as i32
}


// pub fn longest_substring(s: String) -> i32 {
//     let mut result= 0;
//     let mut left_cursor= 0;
//     let mut used: HashMap<char, i32>= HashMap::new();
//     let mut used_two: HashMap<char, i32>= HashMap::new();

//     let mut chars: Vec<char>= s.chars().collect();

//     for (right_cursor, c) in chars.iter().enumerate() {

        
//         // if used.contains_key(c) && left_cursor <= *(used.get(c).unwrap()) {
//         //     left_cursor= used.get(c).unwrap() +1
//         // }else{
//         //     let idx= (right_cursor as i32) - left_cursor+ 1;
//         //     result= cmp::max(result, idx);
//         // }
//         // used_two.insert(*c, right_cursor as i32);
//     }

//     result
// }

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
