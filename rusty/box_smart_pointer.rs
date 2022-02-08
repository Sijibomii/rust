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

struct CustomSmartPointer {
  data: String,
}
// impl the drop trait for the struct
impl Drop for CustomSmartPointer {
  fn drop(&mut self){
    println!("DROPPING CUSTOM SMART POINTER DATA {}", self.data);
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
  // This works fine despite the fact that hello is expecting the pointer to a string slice but we're passing
  // in the pointer to MyBox. This is bc of the automatic deref cohesion in rust
  //it automatically helps us deref it to the point we get a str slice
  hello(&m);

  //without the ADC we would have written it this way
  //derefing m to a string first and passing  a ref to a slice of that str (slice is the full hence [..])
  hello(&(*m)[..]);

  // After this pointers go out of scope, rust automatically calls the drop fn impl in the drop trait
  //to impl our drop logic
  let c = CustomSmartPointer{
    data: String::from("my stuff"),
  };

  let d = CustomSmartPointer{
    data: String::from("other stuff"),
  };

}

fn hello(name: &str){
  println!("Hello {}", name);
}