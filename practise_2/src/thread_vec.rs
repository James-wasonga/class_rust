use std::thread;
use std::time::Duration;

pub fn thread_vec(){
    let v = vec![1,2,3,4];

    let handle = thread::spawn(move||{
        println!("Hello {:#?}",v);
    });
    handle.join().unwrap();
}
