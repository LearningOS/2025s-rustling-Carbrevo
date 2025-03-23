/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T: Ord> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}
#[derive(Debug)]
struct LinkedList<T: Ord> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: Ord> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn divide_by(
        list_a: LinkedList<T>,
        list_b: LinkedList<T>,
    ) -> (LinkedList<T>, Option<LinkedList<T>>, Option<LinkedList<T>>) {
        let value = unsafe { &(*list_b.start.unwrap().as_ptr()).val };

        let mut end = None;
        let mut length = 0;
        let mut next = list_a.start;
        while let Some(next_node) = next {
            if unsafe { (*next_node.as_ptr()).val > *value } {
                break;
            }
            end = next;
            length += 1;
            next = unsafe { (*next_node.as_ptr()).next };
        }

        match next {
            None => (
                LinkedList::<T> {
                    length,
                    start: list_a.start,
                    end,
                },
                None,
                Some(list_b),
            ),
            _ => (
                LinkedList::<T> {
                    length,
                    start: list_a.start,
                    end,
                },
                Some(LinkedList::<T> {
                    length: list_a.length - length,
                    start: next,
                    end: list_a.end,
                }),
                Some(list_b),
            ),
        }
    }

    pub fn split(
        list_a: LinkedList<T>,
        list_b: LinkedList<T>,
    ) -> (LinkedList<T>, Option<LinkedList<T>>, Option<LinkedList<T>>) {
        let node_a = list_a.start.unwrap();
        let node_b = list_b.start.unwrap();
        if unsafe { (*node_a.as_ptr()).val <= (*node_b.as_ptr()).val } {
            Self::divide_by(list_a, list_b)
        } else {
            Self::divide_by(list_b, list_a)
        }
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        let mut merged;
        let mut list_a_rem;
        let mut list_b_rem;

        (merged, list_a_rem, list_b_rem) = Self::split(list_a, list_b);
        while list_a_rem.is_some() || list_b_rem.is_some() {
            if list_a_rem.is_none() {
                let list_b = list_b_rem.unwrap();
                merged += list_b;
                break;
            } else if list_b_rem.is_none() {
                let list_a = list_a_rem.unwrap();
                merged += list_a;
                break;
            }

            let mut merged_next;
            let list_a = list_a_rem.unwrap();
            let list_b = list_b_rem.unwrap();
            (merged_next, list_a_rem, list_b_rem) = Self::split(list_a, list_b);
            merged += merged_next;
        }

        merged
    }
}

impl<T: Ord> std::ops::AddAssign for LinkedList<T> {
    fn add_assign(&mut self, rhs: LinkedList<T>) {
        if rhs.length == 0 {
            return;
        }

        self.length += rhs.length;
        match self.end {
            None => {
                self.start = rhs.start;
                self.end = rhs.end;
            }
            Some(node_last) => {
                unsafe { (*node_last.as_ptr()).next = rhs.start };
                self.end = rhs.end;
            }
        }
    }
}

impl<T: Ord> Display for LinkedList<T>
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

impl<T: Ord> Display for Node<T>
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

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
