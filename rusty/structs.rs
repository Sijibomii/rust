struct User{
  username: String,
  email: String,
  loggin_count: u64,
  active: bool,
}
#[derive(Debug)]
struct Rect{
  width: i32,
  height: i32,
}
//creating an implementation for the rect struct
impl Rect{
  //methods can be defined in here
  //first arg on a method is always self. This is the instance the methods is always called on
  fn area(&self) -> u32{
    self.height * self.width
  }
  //to check if a rect can hold another rect inside it
  fn can_hold(&self, other: &Rect) -> bool{
    self.width > other.width && self.height > other.height
  }
}

impl Rect{
  // We can also create a function associated with the struct
  // associated functions do not get self as first parameter
  fn square(size: u32) -> Rect{
    Rect{
      width: size,
      height: size
    }
  }
}
fn main(){
  let mut user1 = User{
    email: String::from("example@email.com"),
    username: String::from("@username"),
    loggin_count: 40,
    active: true
  };

  let name = user1.username;
  user1.username = String::from("llll");
  let user2 = build_user(String::from("user@ex.com"), String::from("user"));
  let user3 = User{
    username: String::from("userr"),
    ...user1
  };
  let rect1= Rect{
    width: 32,
    height: 44,
  };

  let rect2 = Rect{
    width: 3,
    height: 4,
  };
  // TUPLE STRUCTS
  struct Color(i32, i32, i32);

  // DERIVED TRAITS
  // If we try printing the user struct 
  //println!("{}", user1); //ERROR because we've not implemented the debug trait that allows for printing
  println!("{:#?}", rect1);

  //METHODS. methods are simiplar to functions, exept that they are tied to a struct
  println!("area is: {}", rect1.area());
  println!("rect can hold: {}", rect1.can_hold(&rect2));
  let rect3 = Rect::square(5);
}

fn build_user(email: String, username: String) -> User{
  User{
    email: email,
    username: username,
    loggin_count: 0,
    active: true,
  }
}