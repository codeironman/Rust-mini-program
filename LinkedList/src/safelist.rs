use std::sync::{Arc, Mutex};

type Link<T> = Option<Arc<Mutex<ListNode<T>>>>;

struct ListNode<T> {
    data : T,
    next : Link<T>
}


impl<T> ListNode<T> {
    pub fn new(x : T) -> Self {
        ListNode{
            data : x,
            next : None,
        }
    }
    pub fn insert(&mut self, data : T) {
        let mut p = &mut self.next;
        while let Some(node) = p {
            let p = &mut node.lock().unwrap().next;
        }
        *p = Lin

    }

    pub fn delete(&mut self, x : T) {

    }

}
