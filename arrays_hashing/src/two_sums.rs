pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if nums[i] + nums[j] == target && j != i {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::two_sums::Solution;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(Solution::two_sum(vec![2,4,6,7], 10), vec![1,2])
    }
}
