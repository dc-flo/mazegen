use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub x: u32,
    pub y: u32,
    pub paths: Vec<&'a Rc<RefCell<Node<'a>>>>
}

impl<'a> Node<'a> {
    pub fn new(x: u32, y: u32) -> Self {
        let paths = Vec::new();
        Node {
            x,
            y,
            paths
        }
    }
}