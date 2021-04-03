use std::{cell::RefCell, rc::Rc};

pub struct LinkedList<T: Clone + Default> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}
#[derive(Clone)]
pub struct Node<T: Clone + Default> {
    pub value: T,
    next_node: Option<Rc<RefCell<Node<T>>>>,
    prev_node: Option<Rc<RefCell<Node<T>>>>,
}
pub struct LinkedListIter<T: Clone + Default> {
    head: Option<Rc<RefCell<Node<T>>>>,
    cur_node: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone + Default> Node<T> {
    fn new() -> Node<T> {
        Node {
            value: T::default(),
            next_node: None,
            prev_node: None,
        }
    }
    pub fn next(&self) -> Node<T> {
        if self.next_node.is_some() {
            let next_ref = self.next_node.as_ref();
            let next_unwrap = next_ref.unwrap();
            let next_borrow = next_unwrap.borrow();
            next_borrow.clone()
        } else {
            panic!("No `next` available!");
        }
    }
    pub fn prev(&self) -> Node<T> {
        if self.prev_node.is_some() {
            let prev_ref = self.prev_node.as_ref();
            let prev_unwrap = prev_ref.unwrap();
            let prev_borrow = prev_unwrap.borrow();
            prev_borrow.clone()
        } else {
            panic!("No `prev` available!");
        }
    }
    pub fn mutate(&self, value: T) {
        let next_node = self.next_node.clone();
        let this_node = next_node.unwrap().borrow().prev_node.clone().unwrap();
        this_node.borrow_mut().value = value;
    }
}

impl<T: Clone + Default> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
        }
    }
    pub fn add(&mut self, value: T) {
        let mut new_node = Node::<T>::new();
        new_node.value = value;
        let link = Rc::new(RefCell::new(new_node));
        let mut node_ref = link.borrow_mut();
        if self.head.is_none() {
            node_ref.next_node = Some(link.clone());
            node_ref.prev_node = Some(link.clone());
            self.head = Some(link.clone());
            self.tail = Some(link.clone());
        } else {
            let head = self.head.take().unwrap();
            let tail = self.tail.take().unwrap();
            node_ref.prev_node = Some(tail.clone());
            node_ref.next_node = Some(head.clone());
            head.borrow_mut().prev_node = Some(link.clone());
            tail.borrow_mut().next_node = Some(link.clone());
            self.head = Some(head);
            self.tail = Some(link.clone());
        }
    }
    pub fn head(&self) -> Node<T> {
        if self.head.is_none() {
            panic!("`LinkedList` is not built!");
        }
        let head_link = self.head.clone();
        let head_unwrap = head_link.unwrap();
        let head_ref = head_unwrap.borrow();
        head_ref.clone()
    }
    pub fn tail(&self) -> Node<T> {
        if self.tail.is_none() {
            panic!("`LinkedList` is not built!");
        }
        let tail_link = self.tail.clone();
        let tail_unwrap = tail_link.unwrap();
        let tail_ref = tail_unwrap.borrow();
        tail_ref.clone()
    }
    pub fn is_tail(&self, node: Node<T>) -> bool {
        let next_node = node.next_node;
        let next_unwrap = next_node.unwrap().clone();
        let cur_node = next_unwrap.borrow().prev_node.clone().unwrap();
        if Rc::ptr_eq(&self.tail.clone().unwrap(), &cur_node) {
            true
        } else {
            false
        }
    }
    pub fn is_head(&self, node: Node<T>) -> bool {
        let next_node = node.next_node;
        let next_unwrap = next_node.unwrap().clone();
        let cur_node = next_unwrap.borrow().prev_node.clone().unwrap();
        if Rc::ptr_eq(&self.head.clone().unwrap(), &cur_node) {
            true
        } else {
            false
        }
    }
    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter {
            head: self.head.clone(),
            cur_node: self.head.clone(),
        }
    }
}

impl<T: Clone + Default> Iterator for LinkedListIter<T> {
    type Item = Node<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_node.is_none() {
            None
        } else {
            let cur_t = self.cur_node.clone().unwrap().borrow().clone();
            let next = self.cur_node.clone().unwrap().borrow().clone().next_node;
            if Rc::ptr_eq(&self.head.clone().unwrap(), &next.clone().unwrap()) {
                self.cur_node = None;
            } else {
                self.cur_node = next;
            }
            Some(cur_t)
        }
    }
}

impl<T: Clone + Default> DoubleEndedIterator for LinkedListIter<T> {
    fn next_back(&mut self) -> Option<Node<T>> {
        if self.cur_node.is_none() {
            None
        } else {
            let next_back = self.cur_node.clone().unwrap().borrow().prev_node.clone();
            if Rc::ptr_eq(&self.head.clone().unwrap(), &next_back.clone().unwrap()) {
                self.cur_node = None;
            } else {
                self.cur_node = next_back.clone();
            }
            Some(next_back.clone().unwrap().borrow().clone())
        }
    }
}
