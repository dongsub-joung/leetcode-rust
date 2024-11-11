// https://leetcode.com/problems/two-sum/

struct Solution{}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32>= Vec::new();
        let mut idx_next_e=1;
        let next_nums= nums.clone();
        for (i,j) in nums.iter().enumerate(){
            if j+ next_nums[idx_next_e] == target {
                result.push(i.try_into().unwrap());
                result.push(idx_next_e as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::two_sum(Vec::from([2_i32,7,11,15]), 9_i32);
        assert_eq!(result, Vec::from([0,1]));
    }
}
