
//using loop
use std::io;

fn main() {
    println!("Enter an integer:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the value in the line");

    let num: i32 = input.trim().parse().expect("Please enter a valid integer");

    let mut i = 0;
    loop {
        if i > num {
            break;
        }
        println!("{}", i);
        i += 1;
    }
}

 use std::io;

fn main(){
  println!("Enter an integer");
  
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read the value");

  let num: i32 = input.trim().parse().expect("Please enter a valid input");
   for i in 0.. num {
      println!("{}", i);
   }

}

use std::io;

fn main() {
    println!("Enter an integer:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num: i32 = input.trim().parse().expect("Please enter a valid integer");

    let mut i = 0;
    while i <= num {
        println!("{}", i);
        i += 1;
    }
}
