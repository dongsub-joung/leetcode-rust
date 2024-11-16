struct Solution{}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // "LVIII" -> "L", "V" ...
        let s_chars: Vec<char>= s.chars()
            .map(|f| f)
            .collect();

        let mut result_sum= 0;
        for e_c in s_chars {
            let score= match e_c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => continue,
            };
            result_sum+= score;
        }

        result_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::roman_to_int("LVIII".to_string());
        assert_eq!(result, 58);
        
    }
    #[test]
    fn it_works2() {
        let result = Solution::roman_to_int("MCMXCIV".to_string());
        assert_eq!(result, 1994);
    }
}
