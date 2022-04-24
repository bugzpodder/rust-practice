#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        fn find_median(nums1: &Vec<i32>, nums2: &Vec<i32>) -> Option<f64> {
            let vec: Vec<u32> = (0..nums1.len() as u32).collect();
            let rank = nums1.len() + nums2.len();
            let median = (rank + 1) / 2;
            let result = vec.binary_search_by_key(&median,|&c1| {
                let x = nums1[c1 as usize];
                let upper_idx = nums2.partition_point(|&y| y <= x);
                let lower_idx = nums2.partition_point(|&y| y <= x - 1);
                let result = c1 as usize + upper_idx + 1;
                if c1 as usize + lower_idx + 1 <= median && median <= result {
                    return median;
                }
                return result;
            });
            match result {
                Ok(c1) => {
                    if rank % 2 == 0 {
                        let upper_idx = nums2.partition_point(|&y| y <= nums1[c1]);
                        let mut candidates = Vec::new();
                        if c1 + 1 < nums1.len() {
                            candidates.push(nums1[c1 + 1]);
                        }
                        if upper_idx < nums2.len() {
                            candidates.push(nums2[upper_idx]);
                        }
                        if c1 + upper_idx + 1 > median && upper_idx > 0 {
                            candidates.push(nums2[upper_idx - 1]);
                        }
                        let v2 = *candidates.iter().min().unwrap();
                        return Some((nums1[c1] + v2) as f64 / 2.0);
                    } else {
                        return Some(nums1[c1] as f64);
                    }
                }
                _ => return None
            }
        }

        match find_median(&nums1, &nums2) {
            Some(x) => return x,
            None => return find_median(&nums2, &nums1).unwrap()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,3], vec![2]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,2], vec![3,4,5]), 3.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,2,3,5,8], vec![2,7]), 3.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]), 0.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![0, 0, 0, 0, 0], vec![-1, 0, 0, 0, 0, 0, 1]), 0.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![2,3]), 2.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![2, 3], vec![]), 2.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![2]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![2, 3], vec![1, 4]), 2.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 1], vec![1, 2]), 1.0);
    }
}