// // fn main() {

// //     let square: fn(i32) ->i32 = |x: i32| ->i32{
// //         x*x
// //     };

// //     let sample =|| ->(){
// //         println!("Test");
// //     };

// //     let result:i32 = square(4);
// //     println!("Result is {}",result);

// //     sample();
// // // }


// // fn main(){
// //     let test: i32 = 3;

// //     let multiply: fn(i32) -> i32 = |x: i32| ->i32{
// //         x* test
// //     };

// //     let res: i32 = multiply(10);
// //     println!("{}",res);
// // }


// // fn main(){

// //     let age: String = "50".to_string();

// //     let display_age = move||{
// //         println!("Your age is {}",age)
// //     };

// //     println!("Hello user of age {}", age);
// //     display_age()
// // }


// fn main(){
//   let multiple = |x:i32| -> i32{
//     x* 3
//   };

//     let res = multiply_by_three(multiple);
//     printtln!("Res is {}",res);
//     let double = create_multiples(3);
//     println!("1.double {}",double(40))
  
// }

// fn multiply_by_three<K>(par_1)-> i32 where K:Fn(i32) -> i32{
//    par_1(10)
// }

// fn create_multiples(factor:i32) -> impl Fn(i32) ->i32{
//     move |i32| {
//         i * factor
//     }
// }

// fn create_multiples_2(factor:i32) -> Box<dyn Fn(i32) ->i32{
//     Box::new(|x:i32| x+1)
// }



// struct person containig two properties , name & age
//create multiple instances of person struct
//find the first  person who is older than 30ys

mod names;
use names::Person;

fn main(){

    // let my_name = Person::new("James".to_string(),32);
    let my_name = Person{name:"James".to_string(),age:32};
    // let my_sec = Person::new("Felix".to_string(),35);

    let mut my_vec = Vec::new();
    my_vec.push(my_name);
    // my_vec.push(my_sec);

    let my_find  = my_vec.into_iter().find(|a|{a.age > 30}).unwrap();
        println!("{:?}",my_find);
    
}


