```rust
use std::rc::{Rc, Weak};

struct List {
    head: Option<Box<Node>>,
}

impl List {
    fn new() -> Self {
        Self { head: None }
    }
    fn push(&mut self, id: i32) {
        let new_node = Box::new(Node {
            id,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
}

struct Node {
    id: i32,
    next: Option<Box<Node>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("> drop Node {}", self.id);
    }
}

// n > 0
// 返回一个循环n次引用的智能指针
// 数字从 1 - n
// 1 -> 2 -> 3 -> 4 -> ... -> n -> 1
fn generate_n_loop_pointer(n: usize) -> List {
    let mut list = List::new();
    for i in 0..n as i32 {
        list.push(i + 1);
    }
    list
}

struct WeakList {
    head: Option<Weak<WeakNode>>,
}

impl WeakList {
    fn new() -> Self {
        Self {
            head: Some(Weak::new()),
        }
    }
    fn push(&mut self, id: i32) {
        let new_node = Rc::new(WeakNode {
            id,
            next: self.head.take(),
        });
        self.head = Some(Rc::downgrade(&new_node));
    }
    ///
    // fn pop() {}

    fn get_head(self) -> Option<Weak<WeakNode>> {
        self.head
    }
}

// Drop 打印 id
struct WeakNode {
    id: i32,
    next: Option<Weak<WeakNode>>,
}

impl Drop for WeakNode {
    fn drop(&mut self) {
        println!("> drop WeakNode {}", self.id);
    }
}

// n > 0
// 返回一个循环n次引用的智能指针
// 数字从 1 - n
// 1 -> 2 -> 3 -> 4 -> ... -> n -> 1
fn generate_n_loop_weak_pointer(n: usize) -> WeakList {
    let mut list = WeakList::new();
    for i in 0..n as i32 {
        list.push(i + 1);
    }
    list
}

fn main() {
    let node = generate_n_loop_pointer(10);
    let node = generate_n_loop_weak_pointer(10);
}
```