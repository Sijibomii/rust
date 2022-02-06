//generics allows a function to take in different types of argument.
//the T generic could be any type and allows our function to accepts multiple ranges of types
//This specifies that our generic type should be a type that implements the PartialOrd and copy trait
fn get_largest<T: PartialOrd + Copy>(numbers: Vec<T>) -> T {

}
//GENERICS WITH STRUCTS
struct Point<T, U>{
  x: T,
  y: U,
}

impl<T, U> Point<T, U>{
  fn  x(&self) -> &T{
    &self.x
  }
}

//we can also impl with concrete type
impl Point<i32, i32>{
  //any method defined here will be available to only points where both x and y are of i32 type
}

fn main(){
  let p1 = Point{ x:5, y: 10};
  let p2 = Point{ x:5, y: 10.3 };
}