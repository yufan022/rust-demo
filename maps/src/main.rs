#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}
pub struct Solution;
impl Solution {
  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    // 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
    // 输出：7 -> 0 -> 8
    // 原因：342 + 465 = 807
    let (mut l1, mut l2) = (&l1, &l2);
    let mut result = ListNode::new(0);
    let mut r = &mut result;
    let mut a = 0;
    let mut b = 0;
    let mut up = false;
    let mut sum = 0;
    loop {
      if l1.is_none() && l2.is_none() && !up {
        break;
      }
      a = 0;
      b = 0;
      if let Some(node) = l1 {
        a = node.val;
        l1 = &node.next;
      }else {
          // r.next = Some(Box::new(ListNode::new(1)));
      }
      if let Some(node) = l2 {
        b = node.val;
        l2 = &node.next;
      }else {
          // r.next = Some(Box::new(ListNode::new(1)));
      }

      sum = a + b;
      if up {
        sum += 1;
        up = false;
      }
      if sum >= 10{
        sum = sum - 10;
        up = true;
      }
      r.val = sum;
      if (l1.is_some() && l2.is_some()) || up{
        r.next = Some(Box::new(ListNode::new(0)));
        r = r.next.as_mut().unwrap();
      }else if(l1.is_none() && l2.is_some()) || (l2.is_none() && l1.is_some()) {
        r.next = Some(Box::new(ListNode::new(0)));
        r = r.next.as_mut().unwrap();
      }
    }

    return Some(Box::new(result));
  }
}
fn main() {
  // let mut n0 = ListNode::new(2);
  // let mut n1 = ListNode::new(4);
  // let n2 = ListNode::new(3);
  // n0.next = Some(Box::new(n1));
  // n0.next.as_mut().unwrap().next = Some(Box::new(n2));

  // let mut b0 = ListNode::new(5);
  // let mut b1 = ListNode::new(6);
  // let b2 = ListNode::new(4);
  // b0.next = Some(Box::new(b1));
  // b0.next.as_mut().unwrap().next = Some(Box::new(b2));

  // Solution::add_two_numbers(Some(Box::new(n0)), Some(Box::new(b0)));


  // let mut n0 = ListNode::new(5);

  // let mut b0 = ListNode::new(5);

  // Solution::add_two_numbers(Some(Box::new(n0)), Some(Box::new(b0)));

  let mut n0 = ListNode::new(1);
  let mut n1 = ListNode::new(8);
  n0.next = Some(Box::new(n1));
  let mut b0 = ListNode::new(0);

  Solution::add_two_numbers(Some(Box::new(n0)), Some(Box::new(b0)));
}
