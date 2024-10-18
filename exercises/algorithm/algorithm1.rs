/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: std::cmp::PartialOrd + Clone> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::cmp::PartialOrd + Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => {
                self.start = node_ptr;
                self.end = node_ptr;
            },
            Some(end_ptr) => {
                unsafe { (*end_ptr.as_ptr()).next = node_ptr };
                self.end = node_ptr;
            },
        }
        self.length += 1;
    }

    pub fn get(&self, index: u32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: u32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        let mut merged_list = LinkedList::new();

        let mut a_current = list_a.start;
        let mut b_current = list_b.start;

        while a_current.is_some() || b_current.is_some() {
            match (a_current, b_current) {
                (Some(a_ptr), Some(b_ptr)) => {
                    let a_val = unsafe { &(*a_ptr.as_ptr()).val }; // Borrow the value
                    let b_val = unsafe { &(*b_ptr.as_ptr()).val }; // Borrow the value

                    if a_val <= b_val {
                        merged_list.add(a_val.clone()); // Clone the value to add it to the new list
                        a_current = unsafe { (*a_ptr.as_ptr()).next }; // Move to the next node in list A
                    } else {
                        merged_list.add(b_val.clone()); // Clone the value to add it to the new list
                        b_current = unsafe { (*b_ptr.as_ptr()).next }; // Move to the next node in list B
                    }
                }
                (Some(a_ptr), None) => {
                    let a_val = unsafe { &(*a_ptr.as_ptr()).val }; // Borrow the value
                    merged_list.add(a_val.clone());
                    a_current = unsafe { (*a_ptr.as_ptr()).next };
                }
                (None, Some(b_ptr)) => {
                    let b_val = unsafe { &(*b_ptr.as_ptr()).val }; // Borrow the value
                    merged_list.add(b_val.clone());
                    b_current = unsafe { (*b_ptr.as_ptr()).next };
                }
                (None, None) => {}
            }
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

// Test module remains unchanged
#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for &value in &vec_a {
            list_a.add(value);
        }
        for &value in &vec_b {
            list_b.add(value);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for (i, &value) in target_vec.iter().enumerate() {
            assert_eq!(value, *list_c.get(i as u32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for &value in &vec_a {
            list_a.add(value);
        }
        for &value in &vec_b {
            list_b.add(value);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for (i, &value) in target_vec.iter().enumerate() {
            assert_eq!(value, *list_c.get(i as u32).unwrap());
        }
    }
}
