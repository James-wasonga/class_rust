//! Welcome to Rust 🦀🦀🦀🦀
//! 
//! This is the Documentation of my Rust Class containing different solutions and examples in each section
//!
//! ## Detailed Introduction
//! 
//! This is a detailed into to all the possible concepts in rust programming language with each example to show case the act in each 
//! 
//! ## Features
//! 
//!
//!  - [x] This crate section allows you to create various  object and how they work
//! 
//! ## Example
//! - `User` with a field name is assigned to a declared variable `p1` then printed out 
//! ```
//! let p1 = User{name: "James".to_string()};
//! println!("{}", p1.name);
//!  
//! ```
//! - To use a method within our program we use `impl` keyword followed by `method name` and curly brackets `{}`
//! ```
//! impl User{
//! 
//! }
//! ```
//! 
//! - The function are written by `fn` keyword followed by a `function name` and `()` which may contain parameters and lastly `{}`which holds the function body 
//! 
//! ```
//! fn walk(name){
//!   println!("User {} is walking",name);
//!     }
//! 
//! ```
//! 
//! -For a function to work it must be called using `function name` and passing argurments to it
//! ```
//! fn walk(name){
//! println!("User {} is walking", name);
//! }
//! walk("James");
//! ```
//! 
//! Welcome to many **important** concepts in rust 🦀💥⚠️
//! 
//! ```
//! let l :i32 = 54;
//! 
//! ```
//! 
//! `rust is now`
//! 
//! 



//structs
/// This struct holds the various fileds that the specific user has 
pub struct User{
    /// The `name` field represent the declared name which is a `String` 
    name: String,
    /// `age ` is signed integer of 32 bits 
    age: i32,
    /// `id_number` field is the id number of the `User` which is of signed integer 
    id_number: i32
}
// struct Cars{
//     name: String,
//     number: i32,
//     model: String,
// }

 impl User{
    pub fn walk(&self){
        println!("User {} is eating",&self.name)
    }

    pub fn eat(&self){
        println!("User {} is eating",&self.name)
    }
    pub fn is_adult(&self) -> bool{
        if self.age > 18 {
            return true;
        }
        return false;
    }
}
///All the Arguments and Functions are called within the `main` to enable the program to run

pub fn main(){
    let James = User{
        name: String::from("James"),
        age: 20,
        id_number: 2456676,
    };


let result:bool = James.is_adult();
println!("is James an adult ? {result}");
}
//     let Erick = User{
//         name: String::from("Erick"),
//         age: 20,
//         id_number: 2456676,
//     };

//     println!("Hello {}", Erick.name);
   

//     let car = Cars{
//         name: String::from("Toyota"),
//         number: 966,
//         model: String::from("Subaru"),
//     };
// }

// fn walk(name:String){
//     println!("User is walking {}",name);
// }

