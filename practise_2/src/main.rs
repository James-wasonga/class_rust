mod thread_vec;
use thread_vec::thread_vec;
mod message;
use message::message;



use std::thread;
use std::time::Duration;

fn main() {
    //creating the thread
   let handle = thread::spawn(||{
    
    for i in 1..20{
        println!("{}",i);
        thread::sleep(Duration::from_millis(1));
         }
     });

    handle.join().unwrap();

   println!("Hi main thread");
   //sleeping the main thread to print the inner thread
//   for i in 0..10 {
//     thread::sleep(Duration::from_millis(1));
//  } 
thread_vec();

message();


}
