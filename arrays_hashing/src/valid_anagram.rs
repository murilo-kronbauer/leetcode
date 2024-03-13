use std::collections::HashMap;

// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        // Solution 1 sorting
        // let mut vec_s: Vec<char> = s.chars().collect();
        // let mut vec_t: Vec<char> = t.chars().collect();

        // vec_s.sort();
        // vec_t.sort();

        // vec_s == vec_t

        // Solution 2 with HashMaps
        let mut map_s: HashMap<char, u16> = HashMap::new();
        let mut map_t: HashMap<char, u16> = HashMap::new();

        for i in 0..s.len() {
            s.chars().for_each(|c| *map_s.entry(c).or_insert(0) += 1);
            t.chars().for_each(|c| *map_t.entry(c).or_insert(0) += 1);
        }

        // for c in s.chars() {
        //     if let Some(q) = map_s.get_mut(&c) {
        //         *q = *q + 1;
        //     } else {
        //         map_s.insert(c, 1);
        //     }
        // }

        // for c in t.chars() {
        //     if let Some(q) = map_t.get_mut(&c) {
        //         *q = *q + 1;
        //     } else {
        //         map_t.insert(c, 1);
        //     }
        // }
        
        for (k, v) in map_s {
            if map_t.get(&k).unwrap() != &v { return false; }
        }

        return true
    }
}



#[cfg(test)]
mod tests {
    use crate::valid_anagram::Solution;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(
            Solution::is_anagram("anagram".to_owned(), "nagaram".to_owned()),
            true
        )
    }
}
