use std::io;

 pub fn inputs(){
    println!("Enter an Integer Number ");

    const input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the input");
    let number: i32 = input.trim().parse().expect("Please Enter an integer number");

    // match number{
    //     Ok(num) => println!("The number is correct {}",num),
    //     Err(_) => println!("Please Enter an integer number"),
    // }


    println!("Enter the first number");
    let input : String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read from the input");
    let num1: i32 = input.trim().parse().expect("Enter an integer number ");


    let num2: i32 = input.trin().parse().expect("The number is not an interger");

let add = num1 + num2;
let subtr = num1 - num2;
let mult = num1 * num2;
let div = num1 / num2;

println!("{}"add);
println!("{}"subtr);
println!("{}"mult);


if num2 == 0{
   println!("The num2 should be a non zero number");
}else{
   println!("{}"div);
}



 }
