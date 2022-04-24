#[allow(dead_code)]
struct Solution;

use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut j = 0;
        let mut lookup = HashSet::new();
        let bytes = s.as_bytes();
        let mut result = 0;
        for i in 0..bytes.len() {
            if lookup.contains(&bytes[i]) {
                while bytes[j] != bytes[i] {
                    lookup.remove(&bytes[j]);
                    j += 1;
                }
                j += 1;
            } else {
                lookup.insert(bytes[i]);
            }
            if lookup.len() > result {
                result = lookup.len();
            }
        }
        return result as i32;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("aabaab!bb".to_string()), 3);
    }
}