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
    }
    
    fn add(i: i32, j: i32) -> i32 {
    //just like elixir, function returns last expression
    i + j
    }