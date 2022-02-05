use std::io;
use rand::Rng;// used to create random numbers
use std::cmp::Ordering;
fn main(){
  println!("Guess the number");
  let secret_number: i32 = rand::thread_rng().gen_range(1, 50);
  println!("secret number is {}", secret_number);
  loop {
        println!("enter your guess");
    // Make the guess variable mutatable so I can store the users guess. 
    // Its initailly an empty string
    let mut guess: String = String::new(); // returns an empty string

    //takes in the user input an stores it in guess
    //I passed a mut ref to guess so read_line can mutate without taking ownweship
    io::stdin()
    .read_line(&mut guess)
    .expect("failed to read line");

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("you guessed {}", guess);
    
    // the cmp compares the guess and secret number and returns a value based on the result
    match guess.cmp(&secret_number){
        Ordering::Less => println!("too small"),
        Ordering::Equal => {
            println!("well done!!");
            break;
        },
        Ordering::Greater => println!("too big"),
    }
  }
  
}