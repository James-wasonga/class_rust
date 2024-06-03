mod use_match;
use use_match::number;
use use_match::traffic;

use std::collections::HashMap;
mod char_counts;
use char_counts::char_check;

mod numbers;
use numbers::get_numbers;

mod lambda;
use lambda::sum;


fn main() {
    let numbers: Vec<i32> = vec![1,2,3,4,5,6,7,8,9];

    println!("The 8th number is {} and the 9th is {} ", numbers[7],numbers[8] );

//alternatively

let mut num = Vec::new();
num.push(1);
num.push(5);
num.push(6);
num.push(7);
num.push(9);
num.push(8);
num.push(2);
num.push(8);
num.push(12);
num.push(10);



println!("{:?}", num);
println!("The number from 7th to 9th are {:?},{:?},{:?}",num[6],num[7],num[8]);
//representing them in the array form 
let sub_vector = &num[6..9];
println!("{:?}",sub_vector);



//calling match.rs file here

//number fn
 number(1);
 number(10);
 number(80);
 number(77);

 //traffic fn
 traffic("Orange");
 traffic("Red");
 traffic("Green");
 traffic("Yellow");
 
 //calling char_count.rs here

 char_check();

 //calling the numbers.rs here
 get_numbers();
 
 //calling the vector function here
 let elements = vec![(2,3),(5,8)];
 let result = sum(elements);
println!("{:?}",result);
}