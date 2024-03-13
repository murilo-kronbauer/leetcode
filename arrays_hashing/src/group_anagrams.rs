use std::{collections::HashMap};

pub struct Solution {}

// ["eat","tea","tan","ate","nat","bat"]
// [["bat"],["nat","tan"],["ate","eat","tea"]]

//

impl Solution {
    pub fn group_anagrams(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u32; 26], Vec<String>> = HashMap::new();

        for s in strings {
            let mut count: [u32; 26] = [0; 26];
            println!("This é o valor {s}");

            for c in s.chars() {
                println!("# {c}");
                count[(c as u32 - 'a' as u32) as usize] += 1;
            }

            if let Some(x) = map.get_mut(&count) {
                x.push(s.clone());
            } else {
                map.insert(count, vec![s]);
            }
        }

        return map.values().map(|x| x.to_owned()).collect::<Vec<_>>();
    }
}
