// use std::fmt::{self, Display, Formatter};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::cmp::PartialEq;

type NodePointer<T> = Rc<RefCell<Node<T>>>;
type NodePointerW<T> = Weak<RefCell<Node<T>>>; // used to prevent circular references between Rc pointers

/// Structure of Node for LinkedList.
///
/// # Attributes
///
/// * `value` - Value of node
/// * `next` - The next node 
/// * `prev` - The previous node 
///
#[derive(Debug)]
pub struct Node<T: Copy + PartialEq> {
    pub value: T,
    pub next: Option<NodePointer<T>>,  //rc
    pub prev: Option<NodePointerW<T>>, //weak
}

impl<T: Copy + PartialEq> Node<T> {
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
    ///     //println("{node}");
    /// }
    /// ```
    /// 
    /// # Returns 
    /// 
    /// * `Node<T>` - The node that was created
    /// 
    pub fn new(_value: T) -> Self {
        Self { 
            value: _value,
            next: None,
            prev: None,
        }
    }
}

#[warn(dead_code)]
fn create_ref_node<T: Copy + PartialEq>(_value: T) -> NodePointer<T> {
    Rc::new(RefCell::new(Node::new(_value)))
}


// egality between Node and Node(PartialEq)
impl<T: Copy + PartialEq> PartialEq<Node<T>> for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.value == other.value
    }

    fn ne(&self, other: &Node<T>) -> bool { 
        self.value != other.value
    }
}

impl<T: Copy + PartialEq> From<Node<T>> for Option<NodePointer<T>> {
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

// impl<T> Display for Node<T>
// where
//     T: Display,
// {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         let mut buffer  = String::from("");
//         let node = self.clone();
//         buffer += node.value.to_string().as_str();
//         write!(f, "{}", buffer)
//     }
// }


/// LinkedList structure.
///
/// # Attributes
///
/// * `length` - Size of list
/// * `head` - The first Node of list
/// * `last` - The last Node of list
/// 
#[derive(Debug)]
pub struct LinkedList<T: Copy + PartialEq> {
    pub length: usize,
    head: Option<NodePointer<T>>,
    last: Option<NodePointer<T>>,
}

impl<T: Copy + PartialEq> LinkedList<T>{
    
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
    /// use rust_algo::collections::LinkedList;
    /// fn main() {
    ///     let mut liste : LinkedList<i32> = LinkedList::new();
    ///     liste.push_front(5);
    ///     //println!("{liste}");
    /// }
    /// ```
    /// 
    /// # Returns 
    ///  
    /// * `LinkedList<T>` - The LinkedList that was created
    ///
    pub fn new()  -> Self {
        Self{
            head: None,
            last: None,
            length: 0, 
        }
    }

    fn push_back_node(&mut self, value : T) {
        let mut new_node = Node::new(value);
        match &mut self.last.take() {
            Some(old_last) => {
                new_node.prev = Some(Rc::downgrade(&old_last));
                // new_node.prev = Some(old_last.clone());
                self.last = new_node.into();
                old_last.borrow_mut().next = self.last.clone();
            }
            None => {
                self.head = new_node.into(); //into : A value-to-value conversion that consumes the input value. The opposite of From.
                self.last =  self.head.clone(); //clone a node
            }
        }
        self.length +=1;
    }
    
    fn pop_back_node(&mut self) -> Option<T> {
        match &mut self.last.take() {
            Some(last) => {
                let mut last = last.borrow_mut();
                let previous = last.prev.take();
                match previous {
                    Some(previous) => {
                        let previous = previous.upgrade();
                        // let previous = Some(previous);
                        if let Some(previous) = previous {
                            previous.borrow_mut().next = None;
                            self.last = Some(previous);
                        }
                    }
                    None => {
                        //The previous is None so there is only one element, the head
                        self.head.take();
                    }
                };
                self.length -=1;
                Some(last.value)
            },
            None => None
        }
    }

    fn push_front_node(&mut self, value : T) {
        let mut new_node = Node::new(value);
        match &mut self.head.take() {
            Some(old_head) => {
                new_node.next = Some(old_head.clone());
                self.head = new_node.into();
                if let Some(head) = &self.head {
                    old_head.borrow_mut().prev = Some(Rc::downgrade(&head));
                    // old_head.borrow_mut().prev = Some(head.clone());
                }
            },
            None => {
                self.head = new_node.into();
                self.last = self.head.clone();
            }
        }
        self.length+=1;
    }

    fn pop_front_node(&mut self) -> Option<T> {
        match &mut self.head.take() {
            Some(head) => {
                let mut head = head.borrow_mut();
                let next = head.next.take();
                match next {
                    Some(next) => {
                        //configure new head
                        next.borrow_mut().prev = None;
                        self.head = Some(next); 
                    }, 
                    None =>{
                        self.last.take();
                    }
                
                };
                self.length -=1;
                Some(head.value)
            },
            None => None
        }
    }

    
    /// Add an element to the back of list
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add in the list
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_algo::collections::LinkedList;
    /// fn main() {
    ///     let mut liste : LinkedList<i32> = LinkedList::new();
    ///     liste.push_back(5);
    /// }
    /// ```
    ///
    pub fn push_back(&mut self, value : T) {
        self.push_back_node(value)
    }
    
    /// Add an element to the back of list
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add in the list
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_algo::collections::LinkedList;
    /// fn main() {
    ///     let mut liste : LinkedList<i32> = LinkedList::new();
    ///     liste.add(5);
    /// }
    /// ```
    ///
    pub fn add(&mut self, value : T) {
        self.push_back_node(value)
    }
    
    /// Removes the last element from a list and returns it, or None if it is empty.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_algo::collections::LinkedList;
    /// fn main() {
    ///     let mut liste : LinkedList<i32> = LinkedList::new();
    ///     assert_eq!(liste.pop_back(), None);
    ///     liste.push_back(5);
    ///     assert_eq!(liste.pop_back(), Some(5));
    /// }
    /// ```
    ///
    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node()
    }
  
    /// Add an element to the front of list
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add in the list
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_algo::collections::LinkedList;
    /// fn main() {
    ///     let mut liste : LinkedList<i32> = LinkedList::new();
    ///     liste.push_front(5);
    ///     assert_eq!(liste.pop_front(), Some(5));
    /// }
    /// ```
    ///
    pub fn push_front(&mut self, value : T) {
        self.push_front_node(value)
    }
    
    /// Removes the first element from a list and returns it, or None if it is empty.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_algo::collections::LinkedList;
    /// fn main() {
    ///     let mut liste : LinkedList<i32> = LinkedList::new();
    ///     assert_eq!(liste.pop_front(), None);
    ///     liste.push_front(5);
    ///     assert_eq!(liste.pop_front(), Some(5));
    /// }
    /// ```
    ///
    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node()
    }

    /// Removes all elements from the LinkedList.
    /// 
    /// Examples
    /// 
    /// ```
    /// use rust_algo::collections::LinkedList;
    /// 
    /// fn main() {
    ///     let mut liste : LinkedList<i32> = LinkedList::new();
    ///     liste.push_back(5);
    ///     liste.clear();
    ///     assert_eq!(liste.length, 0);
    ///     assert_eq!(liste.pop_front(), None);
    /// }
    /// ```
    /// 
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Returns true if self is empty
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_algo::collections::LinkedList;
    /// fn main() {
    ///     let mut liste : LinkedList<i32> = LinkedList::new();
    ///     assert_eq!(liste.is_empty(), true);
    ///     liste.push_back(1);
    ///     assert_eq!(liste.is_empty(), false);
    /// }
    /// ```
    /// 
    pub fn is_empty(&self) -> bool {
        self.length==0
    }

    
    /// Returns len of list
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_algo::collections::LinkedList;
    /// fn main() {
    ///     let mut liste : LinkedList<i32> = LinkedList::new();
    ///     liste.push_back(1);
    ///     assert_eq!(liste.len(), 1);
    /// }
    /// ```
    /// 
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns size of list
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_algo::collections::LinkedList;
    /// fn main() {
    ///     let mut liste : LinkedList<i32> = LinkedList::new();
    ///     liste.push_back(1);
    ///     assert_eq!(liste.size(), 1);
    /// }
    /// ```
    /// 
    pub fn size(&self) -> usize {
        self.length
    }

    /// Returns index of _value or -1 if not in Linkedlist
    /// 
    /// # Arguments
    ///
    /// * `_value` - The value whose index we want to know
    ///
    /// # Examples
    /// 
    /// ```
    /// use rust_algo::collections::LinkedList;
    /// fn main() {
    ///     let mut liste : LinkedList<i32> = LinkedList::new();
    ///     liste.push_back(1);
    ///     liste.push_back(3);
    ///     liste.push_back(2);
    ///     assert_eq!(liste.index_of(3), 1);
    ///     assert_eq!(liste.index_of(83), -1);
    /// }
    /// ```
    /// 
    pub fn index_of(&self, _value : T) -> isize {
        let _value = create_ref_node(_value);
        let mut current = self.head.clone();
        let mut index : isize = 0;
        let mut find :bool = false;
        while current.is_some() && find == false {
            //check next element
            if let Some(ref value) =  current {
                let value_current = value.clone();
                if _value == value_current  {
                    find = true;
                }else{
                    index+=1;
                    current = Rc::clone(&value_current).borrow_mut().next.clone();
                }
            }
        }
        
        if self.is_empty() || find ==false{
        return -1;
        }
        index
    }

}



impl<T: Copy + PartialEq> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_linkedlist() -> LinkedList<i32> {
        
        let mut list : LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);

        list
    }

    #[test]
    fn push_and_pop_back_list() {
        // let mut list = List::new();
        let mut list : LinkedList<i32> = LinkedList::new();
        assert_eq!(list.is_empty(), true);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        assert_eq!(list.is_empty(), false);
        assert_eq!(list.length, 4);

        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn push_and_pop_front_list() {
        let mut list : LinkedList<i32> = LinkedList::new();
        assert_eq!(list.is_empty(), true);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);

        assert_eq!(list.is_empty(), false);
        assert_eq!(list.length, 4);

        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
    #[test]
    fn clear_and_drop_list() {
        let mut list : LinkedList<i32> = LinkedList::new();

        list.push_front(1);
        list.push_front(2);
        list.clear();
        assert_eq!(list.length, 0);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn index_of_element_list() {
        let mut list = create_linkedlist();
        list.add(5);
        list.add(6);
        list.add(8);

        assert_eq!(list.index_of(6), 5);
        assert_eq!(list.index_of(8), 6);
        assert_eq!(list.index_of(88), -1);
        assert_eq!(list.index_of(5), 4);

        list.clear();
        assert_eq!(list.index_of(6), -1);
    }

}

