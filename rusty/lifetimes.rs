// the life time of a variable refers to how long the variable lives for
//lifetimes are represented by the ' symbol
//this means that the lifetime of the return value of the function would be the same as the 
//smallest lifetime between x and y
fn longest<'a> (x: &'a str, y: &'a str) ->&'a str{
  if x.len() > y.len(){
    x
  }else{
    y
  }
}
//NOTE: You can not return the refrence to a value created inside a function
//WRONG
// fn ret()-> & str{
//   strr=String::from("s: &str");
//   return strr.as_str();
// }

//LIFETIME RULES
//  THE COMPILER tries to apply this rules to figure out the lifetime and if after applying all three, it can;t
//  still figure out the output lifetime paramter then it forces us to manually set them
// Each parameter that is a refrence gets its own  lifetime parameter
// if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameter
// if there are multiple lifetime parameter but one of them is &self or &mut self then that lifetime is assigned to
//all the output lifetime parameter

fn main(){
  let string1 = String::from("abcd");
  {
    let string2 = String::from("xyz");
    //here the life time of result is equal to the smaller lifetime btw string1 and string2 which is string2
    let result =longest(string1.as_str(), string1.as_str());
    println!("result is {}", result);
  }
  //println!("result is {}", result); //here result is no longer valid bc it is out of scope
  //since string2 is out of scope

  // STATIC LIFETIME
  // this means that the reference could live as long as the duration of the programme
  // ALL STRING LITERALS HAVE A STATIC LIFETIME
  
}