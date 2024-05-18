// //create a program that takes two imputs 
// divides first imput by the second input 
// then check for specific imputs such as divide by 0 and characters which should not be accepted


use std::io;

fn main(){

    println!("Enter First input");

    let mut  input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the input");
    let num1: u32 = input.trim().parse().expect("Enter a valid First number, it should be a integer");


    println!("Enter Second number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the number");
    let num2: u32 = input.trim().parse().expect("Enter a valid Second number");

    //division

    let result = num2 / num1;

    if num2 == 0 {
        println!("The value of second number should be a non-zero");
    }else{
        println!("The result is {}", result);
    }
}