use std::fmt::Display;

type Link<T> = Option<Box<Node<T>>>;
pub struct Node<T>{
    data : T,
    next : Link<T>,
}


impl<T> Node<T> {
    pub fn new(x : T) -> Self {
        Node{
            data : x,
            next : None,
        }
    }
    pub fn insert(&mut self,x : T){

        let mut current = self;
        while let Some(ref mut next) = current.next {
            current= next;
        }
        let new_node = Box::new(Node::new(x));
        current.next = Some(new_node);
    }   
    pub fn pop(&mut self) -> Option<T> {
        let p = & mut self.next;
        if let Some(node) = p {
            *p = node.next.take();
        }
        None
    }
    pub fn delete(&mut self, x: T)
    where
        T: PartialEq,
    {
        let mut p = &mut self.next;
        while let Some(ref mut node) = p {
            if node.data == x {
                *p = node.next.take();
                break;
            } else {
                p =&mut p.as_mut().unwrap().next; 
            }
        }
    }
    pub fn display(&self) 
    where 
        T : Display
    {
        let mut p = self;
        while let Some(ref next_node) = p.next {
            print!("{} ",p.data);
            p = next_node;
        }
        println!("{}",p.data);
    }

}
