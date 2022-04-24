#[allow(dead_code)]
struct Solution;

use std::str;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 0 {
            return "".to_string();
        }
        let s_bytes = s.as_bytes();
        let mut best = (0, 0);
        for c in 1..s.len()*2 {
            let mut lo = (c-1)/2;
            let mut hi = c - lo;
            while lo < s.len() && hi < s.len() && s_bytes[lo] == s_bytes[hi] {
                lo -= 1;
                hi += 1;
            }
            if hi - lo - 1 > best.1 - best.0 + 1 {
                best = (lo + 1, hi - 1);
            }
        }
        
        return str::from_utf8(&s_bytes[best.0..best.1 + 1]).unwrap().to_string();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
        assert_eq!(Solution::longest_palindrome("".to_string()), "");
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a");
        assert_eq!(Solution::longest_palindrome("ccc".to_string()), "ccc");
        assert_eq!(Solution::longest_palindrome("cbbc".to_string()), "cbbc");
        assert_eq!(Solution::longest_palindrome("cbabc".to_string()), "cbabc");

    }
}