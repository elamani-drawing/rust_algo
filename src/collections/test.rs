use std::fmt::{self, Display, Formatter};
use std::cell::RefCell;
use std::rc::Rc;
use super::List;

type Link<T> = Rc<RefCell<Node<T>>>;

/// Structure of Node for LinkedList.
///
/// # Attributes
///
/// * `value` - Value of node
/// * `next` - The node that follows 
///
#[derive(PartialEq, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Link<T>>,
}

// Make and return a Node mutable pointer
fn create_node_ref<T>(value: T) -> Link<T>{
    Rc::new(RefCell::new(Node::new(value)))
}

impl<T> Node<T> {
    /// Make à Node for LinkedList.
    ///
    /// # Attributes
    ///
    /// * `value` - Value of node
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_algo::collections::Node;
    /// fn main() {
    ///     let node: Node<u32> = Node::new(25);
    /// }
    /// ```
    /// 
    /// # Returns 
    /// 
    /// * `Node<T>` - The node that was created
    /// 
    pub fn new(_value: T) -> Self {
        Self { 
            next: None,
            value: _value 
        }
    }

    /// Create a Node and attach it to the next one.
    ///
    /// # Attributes
    ///
    /// * `_value` - Value of self Node
    /// * `_next` - The node after self
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_algo::collections::Node;
    /// use std::rc::Rc; 
    /// use core::cell::RefCell;
    /// fn main() {
    ///     let node: Node<u32> = Node::new(25);
    ///     let node2: Node<u32> = Node::new_with_next(24, Rc::new(RefCell::new(node)));
    /// }
    /// ```
    /// 
    /// # Returns 
    /// 
    /// * `Node<T>` - The node that was created
    /// 
    pub fn new_with_next(_value: T, _next: Link<T>) -> Self {
        Self { 
            next: Some(_next),
            value: _value 
        }
    }

    // fn display(&self) {
        // println!("-> {}", self.a);
        // match self.next {
        //     Some(ref n) => format!("{} -> ", self),
        //     None => "{}"
        // }
    //     format!("{}", self)
    // }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut buffer  = String::from("");
        let node = self.clone();
        buffer += node.value.to_string().as_str();
        write!(f, "{}", buffer)
    }
}


/// LinkedList structure.
///
/// # Attributes
///
/// * `length` - Size of list
/// * `head` - The first Node of list
/// * `last` - The last Node of list
/// 
#[derive(PartialEq, Debug)]
pub struct LinkedList<T> {
    pub length: u32,
    head: Option<Link<T>>,
    last: Option<Link<T>>,
    iterator : Option<Link<T>>,
}

impl<T> List<T> for LinkedList<T>{
    
    /// Create a LinkedList.
    ///
    /// # Attributes
    ///
    /// * `length` - Size of list
    /// * `head` - The first Node of list
    /// * `last` - The last Node of list
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_algo::collections::List;
    /// use rust_algo::collections::LinkedList;
    /// fn main() {
    ///     let mut liste : LinkedList<i32> = LinkedList::new();
    ///     liste.push_front(5);
    /// }
    /// ```
    /// 
    /// # Returns 
    ///  
    /// * `LinkedList<T>` - The LinkedList that was created
    ///
    fn new()  -> Self {
        Self{
            head: None,
            last: None,
            length: 0, 
            iterator : None,
        }
    }

        
    /// add an element at the beginning of the LinkedList.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add in the list
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_algo::collections::List;
    /// use rust_algo::collections::LinkedList;
    /// fn main() {
    ///     let mut liste : LinkedList<i32> = LinkedList::new();
    ///     liste.push_front(5);
    /// }
    /// ```
    ///
    fn push_front(&mut self, value : T) {
        let new_head: Link<T> = create_node_ref(value);
        match self.head.take() {
            Some(old_head) => {
                new_head.borrow_mut().next = Some(old_head);
                // self.head = Some(new_head);
                self.head =Some(Rc::clone(&new_head));
                self.iterator = Some(Rc::clone(&new_head));
            }
            None => {
                self.head = Some(Rc::clone(&new_head));
                self.iterator =  Some(Rc::clone(&new_head));
                self.last = Some(new_head);
            }
        }
        self.length +=1;
    }
    // fn push_back(&self) -> Void;
    // fn clear(&self) -> Void;
    // fn is_empty(&self) -> Void;
    // fn len(&self) -> u32;
}


impl<T> LinkedList<T> {
    // fn up_date_iterator(&mut self) -> Option<Link<T>> {
    fn up_date_iterator(&mut self) {
        //mise à jour de l'iterateur
        // let Some(iterator) = self.iterator;
        match self.iterator.clone() {
            Some(current) => {
                self.iterator = current.borrow_mut().next.clone(); //current item is next node
            },
            None => {
                // while let Some(current_node_rc) =  self.head.clone() {
                //     self.iterator = Some(Rc::clone(&current_node_rc));
                // }
                //---
                // let head_node = if let Some(head_node) =  self.head.clone() { head_node } else { todo!() };
                // self.iterator = Some(Rc::clone(&head_node));
                self.iterator = self.head.clone(); //curent item is first node
            }
        }
        // self.iterator.clone()
    }
        //
        // self.head.clone()
}


impl<T> Display for LinkedList<T>
where
    T: Display,
{  
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut buffer  = String::from("[ "); //init list structure for display
        let mut current_node_option = self.head.clone();
        //iterate all node
        while let Some(current_node_rc) = current_node_option {
            let current_node = current_node_rc.borrow_mut(); 
            //formating display of value
            // buffer += node.value.to_string().as_str();
            buffer += format!("{} -> ", current_node).to_string().as_str();
            current_node_option = current_node.next.clone();
        }
        buffer += "{} ]";//last element && close list structure
        write!(f, "{}", buffer)
    }
}


impl<T> Iterator for LinkedList<T> {
    type Item = Link<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.up_date_iterator();
        self.iterator.clone()
    }

    
    // type Item = T;

    // fn next(&mut self) -> Option<Self::Item> {
    //     self.up_date_iterator();
    //     let suivant = self.iterator.clone();
    //     let head_node = if let Some(head_node) =  suivant { head_node } else { todo!() };
    //     Some(Rc::clone(&head_node).take().value)
    // }
}


#[cfg(test)]
mod test {
    use std::borrow::Borrow;

    use super::*;
    // use std::rc::Rc; 
    // use core::cell::RefCell;
    // Node test
    #[test]
    fn new_node() {
        let node: Node<i32> = Node::new(25);     
        assert_eq!(node.next, None);   
        assert_eq!(node.value, 25);
        let node2: Node<i32> = Node::new_with_next(24, Rc::new(RefCell::new(node)));
        assert_eq!(node2.value, 24);
        assert_eq!(node2.next.expect("-Error on method new_with_next").borrow().value, 25);
    }

    #[test]
    fn push_front_linkedlist(){
        let mut list : LinkedList<i32> = LinkedList::new();
        assert_eq!(list.length, 0);
        list.push_front(5);
        assert_eq!(list.length, 1);
        assert_eq!(list.head.as_ref().expect("-Error on push_front").borrow().value, 5);
        list.push_front(42);
        assert_eq!(list.head.as_ref().expect("-Error on push_front").borrow().value, 42);
        assert_eq!(list.length, 2);
    }

    #[test]
    fn fmt_node(){
        let node: Node<u32> = Node::new(25);
        assert_eq!(format!("{node}"), "25");
    }
    #[test]
    fn fmt_linkedlist(){
        let mut list : LinkedList<i32> = LinkedList::new();
        list.push_front(5);
        list.push_front(42);
        assert_eq!(format!("{list}"), "[ 42 -> 5 -> {} ]");
    }


    #[test]
    fn iter_test() {
        let mut list : LinkedList<i32> = LinkedList::new();
        list.push_front(5);
        list.push_front(42);
        let arr = &[5, 42];
        
        // for (i, node) in list.into_iter().enumerate() {
        //     // assert_eq!(node.borrow().val, arr[i])
        //     println!("--ici");
        //     assert_eq!(node.borrow().value, arr[i]);
        // }
        // for number in list {
        //     println!("{}", number.as_ref(). .expect("-Error on push_front").borrow().value);
        // }
    }

    // LinkedList test
}