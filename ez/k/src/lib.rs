// https://leetcode.com/problems/longest-common-prefix/description/

pub fn command_string(v: Vec<&str>) -> &str {


    "nothing"
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
