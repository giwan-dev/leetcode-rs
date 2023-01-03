// Definition for singly-linked list.
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

    fn from_number(num: i32) -> Option<Box<ListNode>> {
        let digit = num % 10;
        let mut new_node = Self::new(digit);

        new_node.next = {
            let next_num = num / 10;
            if next_num == 0 {
                None
            } else {
                Self::from_number(next_num)
            }
        };

        Some(Box::new(new_node))
    }

    fn to_number(self) -> i32 {
        let mut multiplier = 1;
        let mut result = 0;
        let mut current = Some(Box::new(self));

        while current.is_some() {
            let curr = current.unwrap();

            result += curr.val * multiplier;

            multiplier *= 10;
            current = curr.next;
        }

        result
    }
}

#[allow(dead_code)]
fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut carry = 0;
    let mut result: Vec<i32> = Vec::new();

    while !l1.is_none() || !l2.is_none() {
        let digit1 = l1.as_ref().map_or(0, |node| node.val);
        let digit2 = l2.as_ref().map_or(0, |node| node.val);
        let sum = digit1 + digit2 + carry;
        let digit = sum % 10;
        carry = sum / 10;

        result.push(digit);

        l1 = l1.map_or(None, |node| node.next);
        l2 = l2.map_or(None, |node| node.next);
    }

    if carry == 1 {
        result.push(carry);
    }

    let mut head_node: Option<Box<ListNode>> = None;
    while !result.is_empty() {
        let digit = result.pop().unwrap();
        let mut new_node = Box::new(ListNode::new(digit));
        new_node.next = head_node;
        head_node = Some(new_node);
    }

    head_node
}

#[test]
fn basic() {
    assert_eq!(ListNode::from_number(1234).unwrap().to_number(), 1234);

    // assert_eq!(
    //     add_two_numbers(ListNode::from_number(243), ListNode::from_number(564))
    //         .unwrap()
    //         .to_number(),
    //     243 + 564
    // );
    // assert_eq!(
    //     add_two_numbers(ListNode::from_number(0), ListNode::from_number(0))
    //         .unwrap()
    //         .to_number(),
    //     0
    // );
    assert_eq!(
        add_two_numbers(ListNode::from_number(9999999), ListNode::from_number(9999))
            .unwrap()
            .to_number(),
        9999999 + 9999
    );
}
