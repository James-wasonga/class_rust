//create a program that uses a run crate that generates 10 numbers and prints the ten numbers in desending order

use std::io;

pub fn get_numbers(){
    println!("Enter an integer");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Cannot read from the line");
    let num: i32 = input.trim().parse().expect("Please Enter an Integer Number");
  

    for i in (1..=num).rev() {
         println!("The number is {}", i);
    }

}





