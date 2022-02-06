//defining a trait
pub trait Summary{
  fn summarize(&self) -> String;
  // in the trait we can specify a default impl of a method that could be overwritten by structs impl the trait
  fn default(&self) -> String{
    String::from("I am a default")
  }
}

pub struct NewsArticle{
  pub author: String,
  pub headline: String,
  pub content: String,
}
//implement summary trait for NewsArticle
impl Summary for NewsArticle{
  fn summarize(&self) -> String{
    format!("{}, by {}", self.headline, self.author)
  }
}

pub struct Tweet{
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for NewsArticle{
  fn summarize(&self) -> String{
    format!("{}, by {}", self.username, self.content)
  }
}

// TRAITS AS PARAMETERS
//this means that the notify function can take in any type that impl the Summary trait
pub fn notify(item: &impl Summary){
  println!("Breaking news! {}", item.summarize());
}
//exactly the same as the func written above
pub fn notify<T: Summary>(item: &T){
  println!("Breaking news! {}", item.summarize());
}
pub fn notify<T: Summary>(item1: &T, item2: &T){
  //takes in any type that impl summary but item1 and item2 types must be the same
}
pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary){
  //takes in any type of item1 that impl Summary and Display but item2's type need to impl just Summary
}
//where clauses can be used to make code more readble
//this explains the traits T and U must impl
fn some_func<T, U>(t: &T, u: &U) -> i32
  where T: Display + Summary,
        U: Debug + Clone
        {

        }
//traits can also be used to specify return types
fn returns_summarizable() -> impl Summary{
  Tweet{
    username: String::from("john"),
    content: String::from("hello world"),
    reply: false,
    retweet: false,
  }
}

impl<T: Display> ToString for T{
  //here you're impl trait ToString for any Type T that implements Display
}

fn main(){
  let tweet = Tweet{
    username: String::from("john"),
    content: String::from("hello world"),
    reply: false,
    retweet: false,
  };

  let article = NewsArticle{
    author: String::from("John Doe"),
    headline: String::from("omo yawa dey oo"),
    content: String::from("cool down "),
  };
  println!("Tweet Summary {}", tweet.summarize());
  println!("Article Summary {}", article.summarize());
  notify(&article);
}


