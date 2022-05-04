```rust
#![allow(unused)]

use std::fmt::Display;
/// 链表数字顺序从小到大
/// push(3)
/// 3
/// push(1)
/// 1 -> 3
/// push(2)
/// 1 -> 2 -> 3
/// pop(2)
/// 1 -> 3

pub struct List {
    pub head: Link,
}

impl std::fmt::Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("List").field("head", &self.head).finish()
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("List").field("head", &self.head).finish()
    }
}

type Link = Option<Box<Node>>;

#[derive(Clone, Debug)]
pub struct Node {
    val: i32,
    next: Link,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("List")
            .field("val", &self.val)
            .field("next", &self.next)
            .finish()
    }
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    // insert a value
    // 如果有相同的数字，仍然插入
    pub fn push(&mut self, val: i32) {
        let mut new_node = Box::new(Node { val, next: None });

        match self.head.as_mut() {
            Some(mut node) => {
                // 比head大直接替换
                if node.val > val {
                    new_node.next = Some(self.head.take().expect("head does not found"));
                    self.head = Some(new_node);
                } else {
                    // 往下找
                    while node.next.is_some() && node.next.as_ref().unwrap().val < val {
                        node = node.next.as_mut().expect("next does not found");
                    }

                    if node.next.is_some() {
                        new_node.next = node.next.take();
                    }
                    node.next = Some(new_node);
                }
            }
            // 空链表直接插入
            None => self.head = Some(new_node),
        }
    }

    // pop 有这个数字 就返回 Some(i32)
    // 没有 就返回None
    // 如果有相同的数字，就删除一个就好了
    pub fn pop_num(&mut self, val: i32) -> Option<i32> {
        // self.head.take().map(|node| {
        //     self.head = node.next;
        //     node.val
        // })
        match self.head.as_mut() {
            Some(mut node) => {
                // 当前节点就找到值直接返回，取值一下节点
                if node.val == val {
                    let r = node.val;
                    self.head = node.next.take();
                    return Some(r);
                }

                // 判断 node.next.val 是否等于 val，等于就不进入循环了
                while node.next.is_some() && node.next.as_ref().unwrap().val != val {
                    node = node.next.as_mut().expect("next does not found");
                }
                match node.next.take() {
                    Some(current_node) => {
                        // current_node.val 等于 val
                        let r = current_node.val;
                        node.next = current_node.next;
                        Some(r)
                    }
                    None => None,
                }
            }
            None => None,
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn into_iter(self) -> IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter(List);

impl Iterator for IntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct IterMut<'a> {
    next: Option<&'a mut Node>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();

            &mut node.val
        })
    }
}

pub struct Iter<'a> {
    next: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();

            &node.val
        })
    }
}

// O(1)
// Iter, IterMut, IntoIterator
// 顺序是从小的数字到大的数字输出

fn main() {}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn push() {
        let mut list = List { head: None };
        // 1~9 随机
        list.push(1);
        list.push(9);
        list.push(3);
        list.push(7);
        list.push(5);
        list.push(4);
        list.push(6);
        list.push(2);
        list.push(8);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 4));
        assert_eq!(iter.next(), Some(&mut 5));
        assert_eq!(iter.next(), Some(&mut 6));
        assert_eq!(iter.next(), Some(&mut 7));
        assert_eq!(iter.next(), Some(&mut 8));
        assert_eq!(iter.next(), Some(&mut 9));
    }

    #[test]
    fn pop() {
        let mut list = List { head: None };
        // 1~9 随机
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list.push(6);

        println!("{}", list);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 4));
        assert_eq!(iter.next(), Some(&mut 5));
        assert_eq!(iter.next(), Some(&mut 6));

        assert_eq!(list.pop_num(6), Some(6));
        assert_eq!(list.pop_num(6), None);

        assert_eq!(list.pop_num(1), Some(1));
        assert_eq!(list.pop_num(1), None);

        assert_eq!(list.pop_num(5), Some(5));
        assert_eq!(list.pop_num(5), None);

        assert_eq!(list.pop_num(2), Some(2));
        assert_eq!(list.pop_num(2), None);

        assert_eq!(list.pop_num(4), Some(4));
        assert_eq!(list.pop_num(4), None);

        assert_eq!(list.pop_num(3), Some(3));
        assert_eq!(list.pop_num(3), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List { head: None };
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
    }
}
```