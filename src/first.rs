use std::mem;

pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: List,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List{head: Link::Empty}
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new (Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }
}
