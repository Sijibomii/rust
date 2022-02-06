enum IpAddrType {
  v4,
  v6,
}

struct IpAddr{
  kind: IpAddr,
  address: String
}
// To store data inside the enum type we add () after the variant
enum IpAddrType2{
  v4(String),
  v6(String),
}

enum Message{
  Quit,
  Move{
    x: i32, y: i32
  },
  Write(String),
  ChangeColor(i32, i32, i32),
}
fn main() {
  let localhost = IpAddr{
    kind: IpAddrType::v4,
    address: String::from("127.0.0.1")
  };

  let localhost2 = IpAddrType2::v4(String::from("127.0.0.1"));
  // there are no null values in rust. To represent null values, options are used.
  //options may or may not be defined
  let y: Option<i32> = Some(5); //when an option is wrapped with some, it is defined
  let z: Option<132> = None; //it is not defined here

  let sum: i32 = 9 + y.unwrap_or(10);// the unwarap is used to specify the default if the option is none
}