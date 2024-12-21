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
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut first_value = 0;
        let mut counter = 0;
        let mut current_node = l1.as_ref();
        while let Some(node) = current_node {
            let nval = node.val;
            first_value += nval as i128 * 10_i128.pow(counter as u32);
            counter += 1;

            current_node = node.next.as_ref();
        }

        let mut second_value: i128 = 0;
        let mut counter2 = 0;

        let mut current_node2 = l2.as_ref();
        while let Some(node) = current_node2 {
            let nval = node.val;
            second_value += nval as i128 * 10_i128.pow(counter2 as u32);
            counter2 += 1;

            current_node2 = node.next.as_ref();
        }

        let sum: i128 = first_value + second_value;

        let sum_vec = sum.to_string().chars().collect::<Vec<char>>();

        let mut l3: Option<Box<ListNode>> = None;
        for number in sum_vec {
            let node = ListNode {
                val: number.to_digit(10).unwrap() as i32,
                next: l3,
            };
            l3 = Some(Box::new(node));
        }

        l3
    }
}

fn main() {
    // l1 = [9]
    // l2 = [1,9,9,9,9,9,9,9,9,9]

    let l1 = Some(Box::new(ListNode { val: 9, next: None }));

    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode {
                                    val: 9,
                                    next: Some(Box::new(ListNode {
                                        val: 9,
                                        next: Some(Box::new(ListNode { val: 9, next: None })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        })),
    }));

    let result = Solution::add_two_numbers(l1, l2);

    println!("{:?}", result);
}
