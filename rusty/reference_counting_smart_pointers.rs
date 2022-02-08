use std::rc::Rc;

enum List {
  Cons(i32, Rc<List>),
  Nil,
}


use crate::List::{Cons, Nil};

fn main(){
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  // calling clone does not copy the data. It only increments the ref count ie list of owners
  //here we want both list b and c to both point to a. We can't just pass in a & ref because b would take ownership
  //of a and it won't be avalible for c. 
  // Reference counting smart pointers only allows multiple references to read the data not mutate it
  let b = Cons(3, Rc::clone(&a));
  let c = Cons(4, Rc::clone(&a));
}