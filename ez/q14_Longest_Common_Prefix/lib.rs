// https://leetcode.com/problems/longest-common-prefix/description/
use std::collections::HashMap;

pub fn command_string(v: Vec<&str>) -> &str {
    let mut idx= 0;
    let mut buf= "";
    let mut hashmap: HashMap<char, i32>= HashMap::new();
    for e in v{
        if idx!=0 {
            let chars_v: Vec<char>= e.chars().collect();
            for e_c in chars_v{
                hashmap.insert(e_c, 0);
            }
        }else if idx == 0{
            buf= e;
        }
        idx+=1;
    }
    
    let mut top= 0_i32; 
    for (k, v) in hashmap{
        if top > v{
            top= v;
        }
        
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = command_string(Vec::from(["flower","flow","flight"]));
        assert_eq!(result, "fl");
    }

    #[test]
    fn it_works2() {
        let result = command_string(Vec::from(["dog","racecar","car"]));
        assert_eq!(result, "");
    }

}
