//https://leetcode.com/problems/add-two-numbers/

extern crate core;

use std::cmp::max;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let l1_values = values(l1);
    let l2_values = values(l2);

    let max_range = max(l1_values.len(), l2_values.len());
    let mut acc = vec![0; max_range];

    let mut forward = 0;
    for ind in 0..max_range {
        let i = l1_values.get(ind).unwrap_or(&0);
        let j = l2_values.get(ind).unwrap_or(&0);
        let sum = *i + *j + forward;
        acc[ind] = sum % 10;
        forward = sum / 10;
    }

    while forward != 0 {
        acc.push(forward % 10);
        forward = forward / 10
    }


    from(acc)
}

fn values(node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut acc: Vec<i32> = vec![];
    let mut root = node;
    while root.is_some() {
        let unwrapped = root.unwrap();
        acc.push(unwrapped.val);
        root = unwrapped.next
    }

    acc
}

fn from(arr: Vec<i32>) -> Option<Box<ListNode>> {
    if arr.len() == 0 {
        return None;
    }
    let mut first = ListNode {
        next: None,
        val: *arr.last().unwrap(),
    };

    for n in arr.iter().rev().skip(1) {
        let node = ListNode {
            next: Some(Box::new(first)),
            val: *n,
        };

        first = node;
    }

    Some(Box::new(first))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let l1 = from(vec![2, 4, 3]);
        let l2 = from(vec![5, 6, 4]);
        let expected = from(vec![7, 0, 8]);

        let result = add_two_numbers(l1, l2);

        assert_eq!(expected, result)
    }

    #[test]
    fn case_2() {
        let l1 = from(vec![0]);
        let l2 = from(vec![0]);
        let expected = from(vec![0]);

        let result = add_two_numbers(l1, l2);

        assert_eq!(expected, result)
    }

    #[test]
    fn case_3() {
        let l1 = from(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = from(vec![9, 9, 9, 9]);
        let expected = from(vec![8, 9, 9, 9, 0, 0, 0, 1]);

        let result = add_two_numbers(l1, l2);

        assert_eq!(expected, result)
    }

    #[test]
    fn case_4(){
        let l1: Option<Box<ListNode>> = None;
        let l2 = from(vec![9, 9, 9, 9]);
        let result = add_two_numbers(l1, l2.clone());

        assert_eq!(l2, result)
    }

    #[test]
    fn case_5(){
        let l1: Option<Box<ListNode>> = None;
        let l2: Option<Box<ListNode>> = None;
        let result = add_two_numbers(l1, l2);

        assert_eq!(None, result)
    }
}

fn main() {
    println!("expected {:?}", from(vec![2, 4, 3]));
}