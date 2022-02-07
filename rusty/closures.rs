std::thread;
std::time::Duration;
//CLOSURES are anonymus functions
//closures can be stored in variables and passed around like normal data
//compiler can infer the input types and the return type of closure even tho we could 
//specify it to be more verbose
// let ex_closure = |num: i32| -> i32 {
//   num
// }
//When the compiler infers the type for us only the first usage type is valid i.e
// i.e the first type of param passed into our closure will become the concrete infered type
// let ex_closure = |num| x //close takes in param and returns
//let s = ex_closure(5); //at this point we can only use integer types for the closure
//let s = ex_closure(String::from("hi")); //THIS BECOMES INVALID
// CLOSURES have access to all varibles that are defined within the scope in which the closure is defined unlike functions
// fn main(){
//   let x: 4;
//   let equal_to = |z| z==x;
//   let y==4;
//   assert_eq!(equal_to(4));// THIS PASSES!
// }


struct Cacher<T>
 where
  T: Fn(u32) -> u32 // ALL CLOSURES impl one of the three fn traits (fn, fnmut, fnOnce)
  // fnOnce takes ownwership of the varible inside the closure environment
  // fn mut mutably borrows values
  //  fn immutabily borrows values
  //above, we basically state that the struct can be used on closures where imput is u32  and output is u32
  //i.e T generic must be a closure with imput of u32 and output of u32
  //NOTE: YOU COULD FORCE THE CLOSURE to take ownership of its input params buy adding the move keyword in front 
  // of the closure defination //   let equal_to = move |z| z==x;
{
  calculation: T,
  value:  Option<i32>,
}

impl<T> Cacher<T>
where
  T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T>{
      Cacher{
        calculation,
        value: None,
      }
    }

    fn value(&mut self, arg: u32) -> u32{
      match self.value{
        Some(v) => v,
        None => {
          let v = (self.calculation)(arg);
          self.value = Some(v);
          v
        }
      }
    }
}

fn generate_workout(intensity: u32, random_number: u32){
  let mut cached_result = Cacher::new(|num| {
    //closure defination
    println!("calculating slowly");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!("pushups, {}", cached_result.value(intensity));
  }else{
    if random_number == 3 {
      println!("take a break");
    }else{
      println!("run for {}", cached_result.value(intensity));
    }
  }

}

fn main(){
    let simulated_intensity = 10;
    let random_number = 20;
    generate_workout(simulated_intensity, random_number);
}