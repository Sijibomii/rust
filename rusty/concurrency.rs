use std::{thread, time::Duration};
use std::sync::{mpsc, Mutex, Arc};

fn main(){
  let handle = thread::spawn(||{
    for i in  1..10{
      println!("{} from spawned thread", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in  1..5{
    println!("{} from main thread", i);
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap();// this waits for our spawned thread to finish executing

  // Channels are ways in which threads in rust can pass messages to one another
  // to create a new channel
  //tx -> transmitter, rx -> receiver
  let (tx, rx) = mpsc::channel();
  // to send messages from more than one channel to the receiver(the main thread) we can clone the tx
  let tx2 = tx.clone();
  thread::spawn(move ||{
    let message = String::from("msg1");
    tx.send(message).unwrap();
  });

  thread::spawn(move ||{
    let message = String::from("msg2");
    tx2.send(message).unwrap();
  });

  let received = rx.recv().unwrap();
  println!("{}", received);

  //mutex is an abbreiv for mutaul exclusion ie we allow just one thread access one piece of data at once
  let m = Mutex::new(5);
  {
    //here we call the lock method to acquire the lock for the mutex
    let mut num = m.lock().unwrap();
    *m = 6; //manipulate m bc we have the lock
  }
  // Arc(Atomic Reference Smart pointer is exactly the same as Rc but it is thread safe)
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];
  for _ 0..10{
    let counter = Arc::clone(&counter);
    //the move keyword here indicates that every variable used in this thread will have its ownership moved to the thread
    let handle = thread::spawn(move ||{
      let mut num = counter.lock().unwrap();
      *num +=1;
    });
    handles.push(handle);
  }

  for handle in handles{
    handle.join().unwrap();
  }
  // result should be 10
  println!("Result: {}", *counter.lock().unwrap());
}