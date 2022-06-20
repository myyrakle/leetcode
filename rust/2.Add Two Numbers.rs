// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let a = to_array(l1,vec![]);
        let b = to_array(l2,vec![]);
        
        let mut carry = 0;
        let mut i = 0;
        let mut sum = vec![];
        
        while a.len() > i 
            || b.len() > i {
            let l = a.get(i)
                .unwrap_or(&0);
            let r = b.get(i)
                .unwrap_or(&0);
                
            let mut result = l + r + carry;
            carry = 0;
            if result >= 10 {
                carry = 1;
                result -= 10;
            }
                
            sum.push(result);
                
            i += 1;
        }
        
        if carry > 0 {
            sum.push(carry);
        }
        
        return from_array(sum);
    }
}

fn to_array(
    node: Option<Box<ListNode>>,
    mut acc: Vec<i32>,
)-> Vec<i32> {
    if let Some(node) = node {
        acc.push(node.val);
        to_array(node.next, acc)
    } 
    else {
        acc
    }
}

fn from_array(
    array: Vec<i32>, 
)-> Option<Box<ListNode>>
{
    let mut root: Option<Box<ListNode>> = None;
    let mut current = &mut None;
    
    for e in array {
        let new_node = Box::new(ListNode::new(e));
        
        if *current == None {
            root = Some(new_node);
            current = &mut root;
        }
        else {
            current
                .as_mut()
                .unwrap()
                .next 
                = Some(new_node);
            current =
                &mut current
                .as_mut()
                .unwrap()
                .next;
        }
    }
    
    return root;
}
