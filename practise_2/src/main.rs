use std::thread;
use std::time::Duration;

fn main() {
    //creating the thread
   let handle = thread::spawn(||{
    println!("Hello inner thread");
   });
    handle.join().unwrap();
   println!("Hi main thread");
   //sleeping the main thread to print the inner thread
//   for i in 0..10 {
//     thread::sleep(Duration::from_millis(1));
//  } 

}
