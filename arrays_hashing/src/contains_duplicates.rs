// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        for num in nums {
            if !set.insert(num) {
                return true;
            };
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::contains_duplicates::Solution;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        )
    }
}
