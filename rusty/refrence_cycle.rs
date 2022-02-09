use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(debug)]
enum List {
  //we use the Rc smart pointers because we want our list to have plently owners
  // we use the RefCell because we want to be able to get a mutate ref to diff owners
  Cons(i32, RefCell<Rc<List>>),
  Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>>{
      match self {
        Cons(_, items) => Some(items),
        Nil => None,
      }
    }
}

// HERE WE CREATE A REF CYCLE BC LIST A REF LIST B AND LIST B REF LIST A
// ref cycle are mem leaks and intoduce bugs
fn main(){
  // we wrap this in the rc smart pointer bc we want it to have multiple owners
  let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
  println!("initial Rc count {}", Rc::strong_count(&a));
  println!("a next item {:?}", a.tail());
  let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
  // ref count of a here will be two because we cloned a to create b
  println!("a Rc count after b creation, {}", Rc::strong_count(&a));
  println!("b initial rc count,{}", Rc::strong_count(&b));// one
  println!("b next item {:?}", b.tail());

  if let Some(link) = a.tail() {
    // here we want to mutate a tail to have a ref to list b
    // we use the borrow_mut to get a mut ref and the deref operator to change the data
    *link.borrow_mut() = Rc::clone(&b);
  }

  println!("b rc count after changing a {}", Rc::strong_count(&b));
  println!("a rc count after changing a {}", Rc::strong_count(&a));

  //THIS line causes a stack overflow bc here the tail of a contains b, but the tail of b also contains a
  // This causes a memory leak bc after main, b gets cleaned up first(varibles are cleaned up in rev order to creation)
  //The b pointer on the stack gets cleaned up, but the actual b data will not get cleaned bc a still holds
  // a ref to that data in its tail same thing for a, when the stack pointer to a gets cleaned up, the actual data 
  //in a remains bc it is still ref in b data on the heap
  println!("a next item {:?}", a.tail());
}