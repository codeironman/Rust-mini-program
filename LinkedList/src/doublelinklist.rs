use core::fmt;
use std::{fmt::Display, sync::{Arc, Mutex}};

type Link<K,V> = Option<Arc<Mutex<Node<K,V>>>>;

#[derive(Clone)]
struct  Node <K : Clone,V : Clone>  {
    key : K,
    value : V,
    prev : Link<K,V>,
    next : Link<K,V>,
}

impl<K,V> Node<K,V>
where 
    K : Clone,
    V : Clone,
{
   pub fn new(key : K, value : V) -> Self{
        Self{
            key : key,
            value : value,
            prev : None,
            next : None,
        }
   } 
}

pub struct linkedlist<K : Clone,V : Clone> {
    head : Link<K,V>,
    tail : Link<K,V>,
}

impl<K,V> linkedlist<K,V> 
where 
K : Clone + Display, V : Clone + Display
{
    pub fn new(&mut self) -> Self{
        Self{
            head : None,
            tail : None
        }
    }    
    pub fn push_back(&mut self, key: K, value: V) {
        let new_node = Arc::new(Mutex::new(Node::new(key, value)));
        if let Some(ref old_tail) = self.tail {
            old_tail.lock().unwrap().next = Some(new_node.clone());
            new_node.lock().unwrap().prev = Some(old_tail.clone());
        } else {
            self.head = Some(new_node.clone());
        }
        self.tail = Some(new_node);
    }
    
    
    pub fn push_front(&mut self,key : K , value: V){
        let new_node = Arc::new(Mutex::new(Node::new(key, value)));
        if let Some(ref old_head) = self.head {
            old_head.lock().unwrap().prev = Some(old_head.clone());
            old_head.lock().unwrap().next = Some(old_head.clone());
        }else {
            self.tail = Some(new_node.clone());
        }
        self.head = Some(new_node);
    }

    pub fn remove(&mut self,index : &Link<K,V>){
        
    }
}

impl<K, V> Display for linkedlist<K, V>
where
    K: Clone + Display,
    V: Clone + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut current = self.head.clone();
        while let Some(node) = current {
            let node = node.lock().unwrap();
            write!(f, "({}, {})", node.key, node.value)?;
            if node.next.is_some() {
                write!(f, " -> ")?;
            }
            current = node.next.clone();
        }
        write!(f, "]")
    }
}

fn main() {
    let mut list = linkedlist { head: None, tail: None };
    list.push_back("a", 1);
    list.push_back("b", 2);
    list.push_back("c", 3);
    println!("{}", list);

    let mut list2 = linkedlist { head: None, tail: None };
    list2.push_front("x", 10);
    list2.push_front("y", 20);
    list2.push_front("z", 30);
    println!("{}", list2);
}
