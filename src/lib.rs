use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub children: RefCell<Vec<Rc<Node>>>,
}
