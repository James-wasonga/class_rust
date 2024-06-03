mod occupation;
mod inputation;
mod vector; 

use occupation::print_occupation;
use occupation::print_number;
use vector::check;
use vector::array_check;
use vector::compare;
use vector::array;




fn main() {
   
let my_vec = vec![1,3,4,6];

let _result_1 = my_vec[1];
// println!("Number {}",result_1);

//when trying to access an element that is not within the array 
let result_2 = my_vec.get(99);
// println!("Number {}",result_2.expect("The result does not exist"));

// performing the same operation using a match which makes it easier
match result_2{
Some(number) => {
    println!("{}", number);
}
None => {
    println!("error the number does not exsist");
    }

}

//concidering a one that returns a character

let my_string: String = "Hello World".to_string();
let result = match my_string.chars().nth(10){
    Some(c) => c.to_string(),
    None => "An error occur".to_string()
};

println!("{}", result);

//calling all the external files here

// let res = print_occupation("Scientist");
// println!("{}", res.expect("The occupation is not in the options available"));

// let res1 = print_number("kenn");
// println!("{}", res1.expect("The number does not exist"));

//calling vector here
let res2 = check(vec![23, 45], 1);
println!("{}",res2.expect("The result is invalid"));

let arr = [1,2,3,4];
let index: usize = 2;
let res3 = array_check(&arr ,index);

// println!("{}",res3.expect("The result is invalid"));

match res3{
    Some(val) =>{
        println!("the value is {}", val);
    }
    None => {
        println!("The result is invalid");
}

}



let result = compare(vec![20,30], 1);
println!("{}",result.expect("The result is invalid"));

let arra = [1,2,3,4];
let reso = array(&arra, 2);

// println!("{}", reso)

match reso {
    Some(val) => {
        println!("{}", val);
    }

    None => {
        println!("The result is invalid ");
    }
}

}


