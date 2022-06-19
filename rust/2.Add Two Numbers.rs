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
        
        let a = a
            .into_iter()
            .rev()
            .map(|e|e.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<i32>()
            .unwrap();
        
        let b = b
            .into_iter()
            .rev()
            .map(|e|e.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<i32>()
            .unwrap();
        
        let sum = a+b;
        
        let sum: Vec<_> = sum
            .to_string()
            .chars()
            .rev()
            .map(
                |e|
                e
                .to_string()
                .parse::<i32>()
                .unwrap()
            )
            .collect();
        
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
            current.unwrap().next 
                = Some(new_node);
            current = &mut current.unwrap()
                .next;
        }
    }
    
    return root;
}
