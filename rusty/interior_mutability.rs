//interior mutability is keeping a mutable value inside an immutable reference.
//Unlike box smart pointers to have a mutable value in a box, the pointer itself must be mutable
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger,
{
  pub fn new(messenger: &T, max: usize ) -> LimitTracker<T>{
    LimitTracker{
      messenger,
      max,
      value: 0,
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;
    let percentage_of_max = self.value as f64 / self.max as f64;
    if percentage_of_max > 1.0 {
      self.messenger.send("Error you have over exceeded your qouta");
    } else if percentage_of_max > 0.9{
      self.messenger.send("You have used up 90%");
    }else if percentage_of_max > 0.75 {
      self.messenger.send("youve used up 75%");
    }
  }
}

//Tests...
#[cfg(test)]
mod tests{
  use super::*;
  use std::cell::RefCell;
  struct MockMessenger{
    sent_messages:  RefCell<Vec<String>>,
  }
  // THE REFCELL smart pointer also enables single ownership like the box smart pointer but it enforces its
//ownership rules at runtime unlike box which enforces its at compile time

  impl MockMessenger{
    fn new() -> MockMessenger{
      sent_messages: RefCell::new(vec![]),
    }
  }

  impl Messenger for MockMessenger{
    fn send(&self, message: &str){
      //even tho we recieve an immutable reference since its in a refcell pointer we can call borrow.mut to mutate it
      //
      self.sent_messages.borrow_mut().push(String::from(message));
    }
  }

  #[test]
  fn it_sends_msg() {
      let mock_messanger = MockMessenger::new();
      let mut limit_tracker = LimitTracker::new(&mock_messanger, 100);
      limit_tracker.set_value(80);
      //here we borrow an immutable ref since we are not mutating
      assert_eq!(mock_messanger.sent_messages.borrow().len(), 1);
  }
}