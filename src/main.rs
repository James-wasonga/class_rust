
// // fn main(){
// //     let values = [1.2, 3.4, 8.6];
// //     let output = array_input(&values);
// //     println!("{}", output);
 
// //  }
// //  fn array_input(par1:&[f32]) -> f32{
// //     let mut index = 0;
// //     let length = par1.len();
// //     let mut totals = 0.0;
// //     while index < length{
// //        index = index + 1;
// //        totals = totals + par1[index];
// //     }
// //  return totals
// //  }

// // fn main(){
// //     let array_1 = [1,2,3,6,5,8];
// //     let total_summation: u32 = summation(&array_1);

// //     println!("The total summation is {}", total_summation);

// //     let mut value: String = String::from("what is this? ");
// //     let mut called_function: &mut String = &mut string_function(&mut value);
// //     println!("{}",called_function);
// // }

// // fn summation(array_1: &[u32]) ->u32{
// //     let mut sum: u32 = 0;
// //     for i in array_1{
// //         sum = sum + 1;
// //     }
// //     sum
// // }
 
// // fn string_function(value: & mut String) -> String {
// //   value.push_str("This is a string");
// //   value.to_string()
    
// // }
// mod sol;

// struct Rectangle{
//     width: u32,
//     height: u32,
// }

//     impl Rectangle {
//         fn area(&self) -> u32 {
//             self.width * self.height
//         }
//     }

//     impl Rectangle {
//         fn can_hold(&self, other: &Rectangle) -> bool {
//             self.width > other.width && self.height > other.height
//         }
    
//     } 
//     fn main(){
//    let rect1 = Rectangle{
//         width: 20,
//         height: 30,
// };

// let rect2 = Rectangle{
//     width: 10,
//     height: 20,
// };
// let rect3 = Rectangle{
//     width: 30,
//     height: 30,
// };

// println!("The area of rectangle is {}",rect1.area());
// println!("Is rect1 larger than rect2 {}",rect1.can_hold(&rect2));
// println!("is rect2 larger than rect3 {}", rect2.can_hold(&rect3));
    
// sol::array_input;
        
//     }

//importing files

// use rust_math::num::{sqrt,gcd};
// fn main(){
//     let result: f32 = sqrt(4.0);
//     let result2: i32 = gcd(10,50);
//     println!("result{result}");
//     println!("result{result2}");


// }





//structs

struct User{
    name: String,
    age: i32,
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

fn main(){
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

