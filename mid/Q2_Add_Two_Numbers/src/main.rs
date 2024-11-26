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
  fn set(node: ListNode) -> Self {
    ListNode {
      next: node.next,
      val: node.val
    }
  }
}
struct Solution{}
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let v= l1.clone().unwrap().val;
        let v2= l1.clone().unwrap().next.unwrap().val;
        let v3= l1.unwrap().next.unwrap().next.unwrap().val;
        let l1_result= v3 * 100 + v2 * 10 + v;
        
        let vv= l2.clone().unwrap().val;
        let vv2= l2.clone().unwrap().next.unwrap().val;
        let vv3= l2.unwrap().next.unwrap().next.unwrap().val;
        let l2_result= vv3 * 100 + vv2 * 10 + vv;

        let last= l1_result+l2_result;
        let vvv_three= last % 1000;
        let vvv_two= last % 100;
        let vvv_one= last % 10;
        
        let node_one= ListNode::new(vvv_one);
        let node_two= ListNode::set(node_one);
        let node_three= ListNode::set(node_two);
        Some(Box::from(node_three))
    }
}
