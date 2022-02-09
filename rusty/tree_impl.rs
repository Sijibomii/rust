use std::cell::RefCell;
use std::rc::{Weak, Rc};

struct Node{
  value: 132,
  // we want the ability to modify a parent from its child so we use ref
  // we dont want our children to own the parent so we use the weak smart pointer
  parent: RefCell<Weak<Node>>,
  // our child vector is wrapped in a RefCell bc we want the ability for parents to mutate their children
  children: RefCell<Vec<Rc<Node>>>,
}

fn main(){

  let leaf = Rc::new(Node{
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
  // strong count is the number of refs that have ownership of the data
  // weak count is the number of refs that don't have ownership of the data
  // at this point the strong count should be 1 bc leaf itself is a strong ref to its data and weak should be 0
  println!("leaf strong  = {} weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

  let branch = Rc::new(Node{
    value: 5,
    children: RefCell::new(vec![Rc::clone(&leaf)]),
    parent: RefCell::new(Weak::new()), 
  });
  //Rc::downgrade(&branch) converts a Rc smart pointer to a weak smart pointer
  *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}