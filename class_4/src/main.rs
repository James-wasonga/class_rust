// use std::collections::HashMap;

// // fn main() {
// //     // let x  = vec![1,2,3,4,];
// //     // // println!("{}", x);
// //     // // for i in x {
// //     // //     println!("{}",i);
// //     // // }

// //     // let mut i = 0;
// //     // //  while i < x.len(){
// //     // //     println!("number is {}", x[i]);
// //     // //     i += 1;
// //     // //  }

// //     // loop{
// //     //     if i > x.len(){
// //     //         break;
// //     //     }
   
// //     //     println!("The number is {}", i);
// //     //         i += 1;
// //     // }
// // let mut x: Vec<&str> = Vec::new();
// // x.push("Hello");
// // x.push("world");
// // x.push(".....");


// // // let finals: usize = x.len().saturating_sub(1);
// // // x.truncate(finals);
// // // println!("after {:?}",x)

// // //mutate each item in the vector 
// // //print out the mutated vec and the original vector


// // // let original_x = x.clone();

// // // for element in &x{
// // //     let  mutated;
// // //     *item = "mutated";
// // // }

// // // println!("Mutated vector {:?}", x);
// // // println!("original vector {:?}", original_x);

// // let date_time_string: &str = "2024-05-23t12:34:56";
// // //find way to split date time

// // let parts: Vec<&str> = date_time_string.split('t').collect();

// // if parts.len() == 2{
// //     let date = parts[0];
// //     let time = parts[1];
// //     println!("Date: {}",date);
// //     println!("Time: {}",time);


// // }else{

// //     println!("Invalid date-time string");

// // }

//    //vec > one datatype
//     //hashmap > key & value

//     let maps: HashMap<String, i32> = HashMap::new();
//     maps.insert("Hello".to_string(),22);
//     maps.insert("world".to_string(),32);
//     maps.insert("Kenya".to_string(),54);
    

//     println!("{:?}",maps);
//     maps.insert("Hello".to_string(),300);

//     println!("{:?}",maps);

//     // for (key: String, value:i32) in maps {
//     //     println!("key {} and value {}",key,value)
//     // }

//     maps.entry("Kenya".to_string().or_insert(500));
//     println!("{:?}",maps);        


// // }



 


//create a struct with vector and harshmap
//call a function of the struct to update vector &prints it out
//call a function of the struct to update harshmap & print out

use std::collections::HashMap;

struct MyStruct {
    vec: Vec<i32>,
    hashmap: HashMap<String, i32>,
}

impl MyStruct {
    // Method to update the vector
    fn update_vector(&mut self, value: i32) {
        self.vec.push(value);
        self.print_vector();
    }

    // Method to print the vector
    fn print_vector(&self) {
        println!("Vector: {:?}", self.vec);
    }

    // Method to update the hashmap
    fn update_hashmap(&mut self, key: String, value: i32) {
        self.hashmap.insert(key, value);
        self.print_hashmap();
    }

    // Method to print the hashmap
    fn print_hashmap(&self) {
        println!("HashMap: {:?}", self.hashmap);
    }
}

fn main() {
    let mut my_struct = MyStruct {
        vec: Vec::new(),
        hashmap: HashMap::new(),
    };

    // Update and print the vector
    my_struct.update_vector(10);
    my_struct.update_vector(20);

    // Update and print the hashmap
    my_struct.update_hashmap("one".to_string(), 1);
    my_struct.update_hashmap("two".to_string(), 2);
}
