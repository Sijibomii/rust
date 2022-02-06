use std::collections::HashMap;
fn main(){
  //vectors
  let mut v: Vec<i32> = Vec::new();
  v.push(1);
  //to create a vector with initial values
  let v2 = vec![1,2,3];
  //Vectors are also stored on the heap, so they'll be dropped when they go out of scope
  {
    let v3 = vec![1,2,3];
  }//here, v3 becomes invalid and all the values in it

  // to get a specific value from the vector
  let third = &v2[2];
  //a more safer way
  match v2.get(2) {
    Some(third) => println!("{}", third),
    None => println!("no third element"),
  }

  //STRINGS
  let s1 = String::new();//creates an empty string
  let s2 = "this ia a string slice";//creates a string slice
  let s3 = s2.to_string(); //changing the slice to an owned string
  let s4 = String::from("s: &str");
  let s5 = String::from("world");

  let s6 = s4 + &s5; //ownership of s4 is moved  ie s4 bcms invalid but ownership of s5 is not moved

  //width of strings can be increased dynamically
  let mut s = String::from("foo");
  s.push_str("bar"); //appends a string slice
  s.push('!');// appends a character

  // HASH MAPS
  let blue = String::from("blue");
  let yellow = String::from("yellow");

  let mut scores = HashMap::new();
  scores.insert(blue, 10);// notice that the ownership of the string blue has been passed into the hashmap
  //since it was not passed by refrence
  scores.insert(yellow, 20);
  let team_name = String::from("blue");
  let score = scores.get(&team_name);
}