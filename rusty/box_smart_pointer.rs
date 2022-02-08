use std::ops::Deref;
enum List{
  //wrapping the List input in a box helps the rust compiler calc the number of space that could be taken up
  //by the enum. The box smart pointer is a fixed sized pointer that could now be stored on the stack even tho it points
  //to data on the heap. Without this, we'll need to know the size of the List inside con to calc the size of the list 
  //enum
  Cons(132, Box<List>),
  Nil,
}

use List::{Cons, Nil};

struct MyBox<T>(T);// a tuple struct that holds one generic value

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
      MyBox(x);
    }
}

impl<T> Deref for MyBox<T>{
  type Target = T;

  // Here we are implementing the dref trait for out MyBox struct. 
  //It takes in the self parameter and returns a pointer to a value of type T(The Tuple value orginally stored)
  fn deref(&self) -> &Self::Target{
    &self.0;
  }
}
fn main(){
  //We pass in the data we want to store on the heap and on the stack we store a pointer or a mem address 
  // to location of the data on the heap
  let b = Box::new(5);
  println!("{}", b);

  //
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3,Box::new(Nil))))));

  // DEREF TRAIT: It allows you to customize the behaviour of the derefence operator
  let x = 5;
  let y =&x;

  assert_eq!(5, x);
  assert_eq!(5, *y);

  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5, x);
  //WE COULD DO THIS BC WE HAVE IMPL THE DEREF TRAIT
  assert_eq!(5, *y);

  // AUTOMATIC DEREF COHESION
  let m = MyBox::new(String::from("This is rust"));
  
}

fn hello(name: &str){
  println!("Hello {}", name);
}