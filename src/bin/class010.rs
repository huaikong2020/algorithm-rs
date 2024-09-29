// 将两个升序链表合并为一个新的 升序 链表并返回
// 新链表是通过拼接给定的两个链表的所有节点组成的
// 测试链接 : https://leetcode.cn/problems/merge-two-sorted-lists/
mod bean;
use bean::ListNode;

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // let (mut list1, mut list2) = (list1, list2);
    //     let mut node = None;
    //     let mut cur = &mut node;
    //     *cur = loop {
    //         match (list1, list2) {
    //             (Some(mut a), Some(mut b)) => {
    //                 if a.val < b.val {
    //                     list1 = a.next.take();
    //                     list2 = Some(b);
    //                     cur = &mut cur.insert(a).next;
    //                 } else {
    //                     list1 = Some(a);
    //                     list2 = b.next.take();
    //                     cur = &mut cur.insert(b).next;
    //                 }
    //             }
    //             (x, y) => break x.or(y),
    //         }
    //     };
    //     node
    let mut dummy = ListNode::new(-1);
    let mut cur = &mut dummy;
    let (mut list1, mut list2) = (list1, list2);
    while let (Some(l1), Some(l2)) = (&list1, list2.as_ref()) {
        let l = if l1.val <= l2.val { &mut list1 } else { &mut list2 };
        cur.next = l.take();
        cur = cur.next.as_mut().unwrap();
        *l = cur.next.take();
    }
    cur.next = list1.or(list2);
    dummy.next

}

fn main() {
  let mut n1 = ListNode::new(1);
  let mut n2 = ListNode::new(2);
  let mut n3 = ListNode::new(3);
  n2.next = Some(Box::new(n3));
  n1.next = Some(Box::new(n2));
  
  let mut h = Some(Box::new(n1));  
  let l = &mut h;
  println!("{:?}",l.take());
  println!("{:?}",h);

}