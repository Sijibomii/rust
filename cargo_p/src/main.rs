use std::convert::TryInto; 
use num::complex::Complex;//num dep 
//(::)=>namespace operator. I.E only the Complex type is imported
use std::time::{Duration, Instant}; 
use std::ops::{Add};
use std::time::{Duration};


//generic types
//can accepts only tyie that implements the base trait Add from std::ops
 fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
  }
 
fn main() {
    let a = 10;
    let b: i32 = 20;//i32 is the type-> 32 bit signed integer
    //types can also bse defined as demonstrated below
    let c = 30i32;//30
    let d = 30_i32;//30
    let e = add(add(a, b), add(c, d));
    let one_million: i64 = 1_000_000;//underscores are for readability and are ignored by the compiler
    println!("{}", one_million.pow(2)); 
    println!("{}", a);
    println!("{:?}",&b);
    println!("{}",c);
    println!("{:?}",&d);
    println!("( a + b ) + ( c + d ) = {}", e);
    //f32 means-> 32bit float
    //u32 means-> unsigned 32bit intger
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
        ];
        // isize, usize Integers that assume the CPU’s “native” width. For example, in 64-bit CPUs,
        //usize and isize will be 64-bits wide.
    println!("{:02}", forty_twos[0]); 
    let three = 0b11;//b represent binary
    let thirty = 0o36;//o represents octal
    let three_hundred = 0x12C;//x represents hexadecimal
 
    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);

    // two different types cannot be compared in rust, one as to be casted as the other for the code to compile
    let a: i32 = 10;//signed
    let b: u16 = 100;//unsigned
    if a < (b as i32) {
    //it is safest to cast the smaller type to a larger one. here, we casted a unsigned 16bit to
    //a signed 32bit
    println!("Ten is less than one hundred.");
    }
    //another method of casting different types.
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
     println!("Ten is less than one hundred.");
     }
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("abc (f32)");
    println!(" 0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!(" 0.3: {:x}", (abc.2).to_bits());
    println!();
 
    println!("xyz (f64)");
    println!(" 0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!(" 0.3: {:x}", (xyz.2).to_bits());
    println!();
     //floating numbers should not be compared for equality bc of the way rust stores them
     assert!(abc.0 + abc.1 == abc.2);//abc.0=>0.1, abc.1=>0.2 
    // assert!(xyz.0 + xyz.1 == xyz.2);//this crashes the program
    

    //this code checks for acceptable margin for equality between numbers
    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let absolute_difference = (desired - result).abs();
    assert!(absolute_difference <= f32::EPSILON);
     
    let x = (-42.0_f32).sqrt();//returns NaN
    println!("{}",x)
    //assert_eq!(x, x);//always crashes bc NaN are never equal

    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im)
    
    let container=[
        45,
        45,
        67
    ]
    //for loops
     //using a reference to the container makes sure the cointainer is still reachable after the loop ends.
    for item in &container {
        // ...
    }
    //&mut is used if you need to modify each item during the loop
    for item in &mut container {
        // ...
       }

    //after this loop ends, the container varible becomes invalid
    for item in container {
        // ...
    }
    //container can no longer be reached here.

    //anonymus loops. when the item is not needed in the loop
    for _ in 0..10 {
        // ...
       }
    
    //while loops
    let mut samples = vec![];
    while samples.len() < 10 {
    let sample = take_sample();
    if is_outlier(sample) {
    continue;
    }
    samples.push(sample);
    }
        

    let mut count = 0; //mut means the data should be changable
    let time_limit = Duration::new(1,0);//creates a duration that lasts 1 second
    let start = Instant::now();
    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);

    //loop continues running until it encounters a break keyword;
    loop {
        // ...
    }
    //A loop label is an identifier prefixed with an apostrophe (')
    'outer: for x in 0.. {
        for y in 0.. {
        for z in 0.. {
        if x + y + z > 100 {
        break 'outer;
        }
        // ...
        }
        }
    }
    let n = 654321;
    let description = match is_even(n) {
        true => "even",
        false => "odd",
    };
    println!("{} is {}", n, description);//prints odd since n is not even

    //match is like if-else but it matches specific values
    match item {
        0 => {},//matches when item is exactly zero
        10 ..= 20 => {},//matches between 10 and 20
        40 | 80 => {},//matches 40 or 80
        _ => {},//matches every other value
    }

    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
 
    for item in &haystack {
    let result = match item {
        42 | 132 => "hit!",
         _ => "miss",
     };

    if result == "hit!" {
        println!("{}: {}", item, result);
     }
    }
    let floats = add(1.2, 3.4);
    let ints = add(10, 20);
    let durations = add(Duration::new(5, 0), Duration::new(10, 0));

    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);

    //arrays
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];//[u8; 3] means and array of size 3 with only unsigned 8 bit allowed
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];
   
    }
    // fn is_outlier()-> bool{
    //     return false
    // }
    // fn take_sample()-> i32{
    //     40//some random number
    // }
    fn add(i: i32, j: i32) -> i32 {
    //just like elixir, function returns last expression
    i + j
    }