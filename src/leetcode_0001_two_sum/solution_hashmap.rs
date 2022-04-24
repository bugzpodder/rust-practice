use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut values: HashMap<i32, usize> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            match  values.get(&(target-v)) {
                Some(j) => return vec![*j as i32, i as i32],
                None => { values.insert(*v, i); }
            }
        }
        return Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
        assert!(Solution::two_sum(vec![3, 2, 5], 6).is_empty());
    }
}