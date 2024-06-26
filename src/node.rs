use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub x: u32,
    pub y: u32,
    pub conn: bool,
    pub paths: Vec<Rc<RefCell<Node>>>,
    pub target: Option<Rc<RefCell<Node>>>
}

impl Node {
    pub fn new(x: u32, y: u32) -> Self {
        let paths = Vec::new();
        Node {
            x,
            y,
            conn: false,
            paths,
            target: None
        }
    }
}