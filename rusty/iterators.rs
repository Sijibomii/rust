fn main(){
  let v1 = vec![1, 2, 3];// creates a vector
  let v1_iter = v1.iter();// creates an iterator out of the vector
  for value in v1_iter{// loops over the iterator
    //here we don't take ownership of the value neither can we mutate it
    // to be able mutate we define the iter as  let v1_iter = v1.iter_mut();
    // to take ownership we define the the iter as et v1_iter = v1.into_iter();
    println!("{}", value);
  }
  // We have two broad category of methods in the iterators trait
  // - Adaptors: They take in an iterator and return another iterator
  // - Consumers: They take in an iterator and return some other type such as integer
  // the sum method is a consumer that returns the sum of all values in an iter let sum = v1_iter.sum()
  // examples of adaptors are map -> they take in an closure and returns another iter which is the result of calling
  // individual elems of the iter 
  let new_iter = v1.iter().map(|x| x + 1); // creates a new iter
  let collect = new_iter.collect();// this changes it back to a vector
}