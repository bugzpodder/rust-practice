// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut result: Option<Box<ListNode>> = None;
        let mut current: &mut Option<Box<ListNode>> = &mut result;
        let mut node1 = &l1;
        let mut node2 = &l2;

        while node1.is_some() || node2.is_some() || carry > 0 {
          if node1.is_some() {
            carry += node1.as_ref().unwrap().val;
            node1 = &node1.as_ref().unwrap().next;
          }
          if node2.is_some() {
            carry += node2.as_ref().unwrap().val;
            node2 = &node2.as_ref().unwrap().next;
          }

          *current = Some(Box::new(ListNode::new(carry % 10)));
          current = &mut current.as_mut().unwrap().next;
          carry = carry / 10;
        }
        return result;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        fn from_vec(l: Vec<i32>) -> Option<Box<ListNode>> {
          let mut result = None;
          let mut current: &mut Option<Box<ListNode>> = &mut result;
          for x in l {
            *current = Some(Box::new(ListNode::new(x)));
            current = &mut current.as_mut().unwrap().next;
          }
          return result;
        }
        fn compute(l1: Vec<i32>, l2: Vec<i32>) -> Vec<i32> {
          let result = Solution::add_two_numbers(from_vec(l1), from_vec(l2));
          let mut node = &result;
          let mut result_vec = Vec::new();
          while node.is_some() {
            result_vec.push(node.as_ref().unwrap().val);
            node = &node.as_ref().unwrap().next;
          }
          return result_vec;
        }
        assert_eq!(compute(vec![2, 4, 3], vec![5, 6, 4]), vec![7, 0, 8]);
        assert_eq!(compute(vec![0], vec![0]), vec![0]);
        assert_eq!(compute(vec![9, 9, 9, 9, 9, 9, 9], vec![9, 9, 9, 9]), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }
}