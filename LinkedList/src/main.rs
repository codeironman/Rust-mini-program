mod node;
mod safelist;
use crate::node::Node;
fn main() {
   let mut LinkedList = Node::new(0);
   LinkedList.display();
   LinkedList.insert(1);
   LinkedList.insert(2);
   LinkedList.display();
   LinkedList.delete(1);
   for i in 5..10{
      LinkedList.insert(i);
   }
   LinkedList.display();
}
