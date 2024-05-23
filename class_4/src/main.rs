fn main() {
    // let x  = vec![1,2,3,4,];
    // // println!("{}", x);
    // // for i in x {
    // //     println!("{}",i);
    // // }

    // let mut i = 0;
    // //  while i < x.len(){
    // //     println!("number is {}", x[i]);
    // //     i += 1;
    // //  }

    // loop{
    //     if i > x.len(){
    //         break;
    //     }
   
    //     println!("The number is {}", i);
    //         i += 1;
    // }
let mut x: Vec<&str> = Vec::new();
x.push("Hello");
x.push("world");
x.push(".....");


// let finals: usize = x.len().saturating_sub(1);
// x.truncate(finals);
// println!("after {:?}",x)

//mutate each item in the vector 
//print out the mutated vec and the original vector


// let original_x = x.clone();

// for element in &x{
//     let  mutated;
//     *item = "mutated";
// }

// println!("Mutated vector {:?}", x);
// println!("original vector {:?}", original_x);

let date_time_string: &str = "2024-05-23t12:34:56";
//find way to split date time

let parts: Vec<&str> = date_time_string.split('t').collect();

if parts.len() == 2{
    let date = parts[0];
    let time = parts[1];
    println!("Date: {}",date);
    println!("Time: {}",time);


}else{

    println!("Invalid date-time string");

}

}
