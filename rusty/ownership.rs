
fn main(){
  //if you don't need ownership of a varible use a refrence to the varible instead
  let a: i32 = 5

  //ownership rules
  //Every value in rust has a varible that is called the owner
  //there can only be one owner at a time
  //when the owner goes out of scope, the value will be dropped

  //Concepts of move
  let s1 = String::from("hello");
  let s2 = s1; //s1 has been moved into s2 i.e this does not automatically copy neither does it shallow copy it moves
  // i.e at this point s1 becomes invalid
  //println!("{}", s1); ERROR! bc s1 is now invalid

  let ss1 = String::from("hiii");
  let ss2 = ss1.clone(); //This now performs a clone copy 

  //SIMPLE TYPES STORED ON THE STACK I.E INTEGERS BOOLEANS CHARACTERS ARE COPIED AUTOMATICALLY NOT MOVED
  let x = 5;
  let y = x; //x here is still valid but y is a copy of x
  println!("{}", x); //valid

  //OWNERSHIP
  let s = String::from("hello world");
  takes_ownership(s);
  //println!("{}", s); INVALID bc the takes_ownership function already took ownership of s and s became invalid after the 
  //function returned. passing in a parameter to a function moves the ownership of the variable to the function

  //IF  you don't want to take ownership pass in a refrence to the variable then you don't have to return ownership
  //when you're done
  let ss = String::from("hello world");
  takes_ownership(&ss);//pass in a refrence
  println!("{}", ss);//valiad

  //TO have the permission to mutate a variable without taking ownership, use the mut key word
  let mut sss = String::from("new string");
  change(&mut sss);

  let x = 5;
  makes_copy(x);
  println!("{}", x)// This is still valid bc just as before simple types get copied not moved!
}
fn makes_copy(number: i32){
  println!("{}", number);
}
//TO RETURN OWNERSHIP OF THE STRING BACK TO CALLING FUNCTION RETURN THE STRING AFTER THE FUNCTION IS DONE
fn takes_ownership(some_string: String){
  println!("{}", some_string);
  // some_string // this moves ownership back to main or whoever called it
}
fn calc_length(some: &Sting) -> i32 {
  let len: i32 = some.len();
  len
}
fn change(st: &mut String){
  st.push_str("noww"); //can be mutated here
  //NOTE: YOU CAN ONLY HAVE ONE MUTABLE REF TO A PIECE OF DATA IN A SCOPE
  //NOTE: YOU CAN'T HAVE A MUTABLE REFRENCE IN AN Immutable refrence already exits in that scope
  //NOTE: YOU CAN HAVE more than one immutable ref tho.
}
