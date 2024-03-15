use core::fmt;
use std::{ fmt::Display, sync::{Arc, Mutex}};


type Link<K,V> = Arc<Mutex<Node<K,V>>>;

#[derive(Clone)]
pub struct  Node <K : Clone,V : Clone>  {
    key : K,
    value : V,
    prev : Option<Link<K,V>>,
    next : Option<Link<K,V>>,
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

pub struct Linkedlist<K : Clone,V : Clone> {
    head : Option<Link<K,V>>,
    tail : Option<Link<K,V>>,
}

impl<K,V> Linkedlist<K,V> 
where 
K : Clone + Display, V : Clone + Display
{
    pub fn new(&mut self) -> Self{
        Self{
            head : None,
            tail : None
        }
    }    
    pub fn push_back(&mut self, key: K, value: V) -> Link<K,V>{
        let new_node = Arc::new(Mutex::new(Node::new(key, value)));
        if let Some(ref old_tail) = self.tail {
            old_tail.lock().unwrap().next = Some(new_node.clone());
            new_node.lock().unwrap().prev = Some(old_tail.clone());
        } else {
            self.head = Some(new_node.clone());
        }
        self.tail = Some(new_node.clone());
        return new_node;
    }
    
    
    pub fn push_front(&mut self,key : K , value: V) -> Link<K,V>{
        let new_node = Arc::new(Mutex::new(Node::new(key, value)));
        if let Some(ref old_head) = self.head {
            old_head.lock().unwrap().prev = Some(new_node.clone());
            new_node.lock().unwrap().next = Some(old_head.clone());
        }else {
            self.tail = Some(new_node.clone());
        }
        self.head = Some(new_node.clone());
        return new_node;
    }

    pub fn pop_back(&mut self) -> Option<Link<K,V>>{
      if let Some(tail) = self.tail.take() {
         let mut tail_node = tail.lock().unwrap();
         if let Some(pre) = tail_node.prev.take() {
            let mut pre_node = pre.lock().unwrap();
            pre_node.next = None;
            self.tail = Some(pre.clone());
         }
         else {
            self.head = None;
         }
         Some(tail.clone())
      }
      else {
         return None;
      }
    }
    pub fn remove(&mut self,index : Link<K,V>){
      let node = index.lock().unwrap();
        if let Some(ref pre_node) = node.prev {
         let mut pre_node = pre_node.lock().unwrap();
         pre_node.next = node.next.clone();
        }
        else{
            self.head = node.next.clone();
        }
        if let Some(ref next_node) = node.next {
            let mut next_node = next_node.lock().unwrap();
            next_node.prev = node.prev.clone();
        }
        else {
            self.tail = node.prev.clone();
        }
    }
}

impl<K, V> Display for Linkedlist<K, V>
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
    let mut list = Linkedlist { head: None, tail: None };
    let a = list.push_back("a", 1);
    let b = list.push_back("b", 2);
    let c = list.push_back("c", 3);
    list.remove(b);
    list.pop_back();
    println!("{}", list);

    let mut list2 = linkedlist { head: None, tail: None };
    list2.push_front("x", 10);
    list2.push_front("y", 20);
    list2.push_front("z", 30);
    println!("{}", list2);
}
