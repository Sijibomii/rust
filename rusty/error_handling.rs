use std::fs::File;
fn main() -> Result<(), Box<dyn Error>>{
  //panic!("Crash") //this is used to cause the program to crash and print out the error given

  //to gracefully handle errors, you could use the Result enum
  let f = File::open("hello.txt");
  let f = match f {
    Ok(file) => file,
    Err(error) => panic!("problem opening the file {:?}", error),
  };

  //using unwrap
  let f = File::open("hello.txt").unwrap();// same as above

  //ERROR PROPAGATION
  let f = File::open("hello.txt")?;// here if this operation fails it returns early with the error even if its not the last line

}