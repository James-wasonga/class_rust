//program to calculate inputs keyed by the users

use std::io;

fn main(){

   println!("Enter First number");
   
 
 let mut input = String::new();
 io::stdin().read_line(&mut input).expect("Failed to read the value in the line ");
 let num1: i32 = input.trim().parse().expect("Enter a valid first number");

 println!("Enter second number");

 let mut input = String::new();
 io::stdin().read_line(&mut input).expect("Failed to read a valid input from the line");
 let num2: i32 = input.trim().parse().expect("Enter a valid second number");

 let addition: i32 = num1 + num2;
 let subtraction: i32 = num1 - num2;
 let multiplication: i32 = num1 * num2;
 let division : i32 = num1 / num2;

 if  num2 == 0 {
   println!("Enter a valid numbers (hint: second number should not be zero )");
 }else{

 println!("The addition is {addition}");
 println!("The subtraction is {subtraction}");
 println!("The multiplication is {multiplication}");
 println!("The division is {division}");

 }
}