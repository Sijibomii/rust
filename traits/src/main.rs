// the "//!" is used to annotate the current item or module  e.g
//! This is code explaining how traits work with enum and structs

#![allow(dead_code)]//allow unused variables. this relaxes the compiler's strictness

use std::fmt;
use std::fmt::{Display};
// The compiler is capable of providing basic implementations for some traits via the #[derive] attribute. 
// These traits can still be manually implemented if a more complex behavior is required.
//debug and PartialEq are types of derivable traits from the fmt. there are others thought
//https://doc.rust-lang.org/rust-by-example/trait/derive.html
//Debug helps to format a value using the {:?} formatter. It defines what is displayed when the {:?} is used against 
//an enum or stuct

//any struct or enum that does not add this derive word can not be printed with the println! 
//struct Seconds(i32);
//let _one_second = Seconds(1);// in this place the defination of the seconds structs lacks the derive keyword
//if you try to print a value of type seconds, it fails
// Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
//println!("One second looks like: {:?}", _one_second);
// TODO ^ Try uncommenting this line

//the PartialEq is used to make types comparable. And defines what is meant by equality
// Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
//let _this_is_true = (_one_second == _one_second);
// TODO ^ Try uncommenting this line
#[derive(Debug,PartialEq)]
pub enum FileState {//pub is used to mark an enum’s variants as public. if left unspecified, it will be made private
    Open,
    Closed,
 }

#[derive(Debug)]//https://stackoverflow.com/questions/46388386/what-exactly-does-derivedebug-mean-in-rust
struct File {
    //each data inside a struct is also private until it is made public
    //pub name: String, -> this is public
    name: String,
    data: Vec<u8>,
    state: FileState,
    }
// the "///" is used to generate documentation about a line of code that follows it. example
///implements the display trait for the filestate enum.
impl Display for FileState {
    //the functions inside the implementation are also private until specified with a pub keyword
    //pub fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{ }
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>",self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main() {
    let f6 = File::new("f6.txt");
    //...
    println!("{:?}", f6);
    println!("{}", f6);
}