// 给你两个 非空 的链表，表示两个非负的整数
// 它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字
// 请你将两个数相加，并以相同形式返回一个表示和的链表。
// 你可以假设除了数字 0 之外，这两个数都不会以 0 开头
// 测试链接：https://leetcode.cn/problems/add-two-numbers/
mod bean;
use bean::ListNode;
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut l1,mut l2) = (l1,l2);
    let mut x = 0;
    let mut ans = ListNode::new(-1);
    let mut cur = &mut ans;
    while let (Some(p1),Some(p2)) = (l1.as_ref(),l2.as_ref()) {
        cur.next = Some(Box::new(ListNode::new((p1.val + p2.val + x) % 10)));
        x = (p1.val + p2.val + x) / 10;
        l1 = p1.next.take();
    }
    return ans.next;
}
fn main() {
    
}