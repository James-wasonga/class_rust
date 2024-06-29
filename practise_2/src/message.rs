use std::thread;
use std::time::Duration;
use std::sync::mpsc;


pub fn message(){
let (tx, rx) = mpsc::channel();


for i in 0..10{
    let sender = tx.clone();
    thread::spawn(move||{
        sender.send(format!("loop number is{}",i));
        thread::sleep(Duration::from_millis(10));

    });
}

for r in rx{
    println!("recieved {}",r );
}


}