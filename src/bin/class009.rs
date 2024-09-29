// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
mod bean;
use bean::ListNode;
//leetcode206
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre = None;
    let mut head1 = head;
    while let Some(mut node) = head1 {
        let next = node.next.take();
        node.next = pre;
        pre = Some(node);
        head1 = next;
    }
    pre
}

// Definition for double-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct DoubleListNode {
    pub val: i32,
    pub next: Option<Box<DoubleListNode>>,
    pub pre: Option<Box<DoubleListNode>>
  }
  
impl DoubleListNode {
    #[inline]
    fn new(val: i32) -> Self {
        DoubleListNode {
        next: None,
        pre: None,
        val
        }
    }
}
  
//rust实现双向链表较为困难，详细见：https://github.com/JasonkayZK/rust-learn/blob/algorithm/collection/src/list/linked_list.rs
// pub fn reverse_double_list(head: Option<Box<DoubleListNode>>) -> Option<Box<DoubleListNode>> {
//     let mut pre = None;
//     let mut head = head;
//     while let Some(mut node) = head {
//         let next = node.next.as_ref();
//         node.next = pre;
//         node.pre = next;
//         pre = Some(node);
//         head = next;
//     }
//     pre
// }


fn main() {
    
}