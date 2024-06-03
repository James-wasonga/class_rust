use std::fmt::Debug;
use std::fmt::Display;



pub fn print_debug<T: Debug>(item: T){
    println!("{:?}",item)
} 

pub fn print_display<A: Display+ std::fmt::Debug>(item: A){
    println!("{:?}",item);
}

// #[derive(Debug)]
// pub struct Culture{
//     name: String
// }