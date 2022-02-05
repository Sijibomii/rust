
fn main(){
  //constants can not be mutated and they must always be initialized with types
  const SUB: i32 = 4000;

  //SHADOWING 
  //shadowing allows to create a new varible with the same name aas before
  let x = 4;
  let x = "six";
  
  // Compound Types
  //tupules
  let tup : (i32, String) = (43, "okk");
  //tuples allow for destucturing
  let (number, word) = tup;
  let num= tup.0; //gets the first element of the tuple
  
}