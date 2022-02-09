use std::{thread, time::Duration};
use std::sync::mpsc;

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
}