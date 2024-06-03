// make a simple calculator that enable users to key input, perform operations and then displays the results

use std::io;

fn main(){

    //first number
    println!("Enter the first number");
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Failed to read the input");
   let num1: u32 = input.trim().parse().expect("Enter a valid Number");


   

   //second number
   println!("Enter the second number");
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Failed to read the input");
   let num2: u32 = input.trim().parse().expect("Please enter an integer number");


   //performing mathematical operations

   let add = num1 + num2;
   let sub = num1 - num2;
   let mult = num1 * num2;
   let div = num1 / num2;

   println!("{}",add);
   println!("{}",sub);
   println!("{}",mult);



   if num2 == 0{
     println!("Second number should not be a zero");
   }else{
    println!("{}",div);
   }
}