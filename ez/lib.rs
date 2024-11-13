struct Solution{}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
//         Symbol       Value
        // I             1
        // V             5
        // X             10
        // L             50
        // C             100
        // D             500
        // M             1000
        
        let s_chars: Vec<char>= s.chars()
            .map(|f| f)
            .collect();

        // "LVIII" -> "L", "V" ...
        

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::roman_to_int("LVIII".to_string());
        assert_eq!(result, 1994);
        
    }
    #[test]
    fn it_works2() {
        let result = Solution::roman_to_int("MCMXCIV".to_string());
        assert_eq!(result, 58);
    }
}
