
mod values;
use values::largest_integer;
use values::largest_char;
// use values::largest;
mod generic_solutions;
use generic_solutions::print_debug;
use generic_solutions::print_display;
// use generic_solutions::Culture;
mod generic_struc;
use generic_struc::Point;
use generic_struc::swap;
//mod vehicle;
//use vehicle::Vehicle;
mod game;
use game::Game;
use game::get_generic;








fn main(){

    //calling the function largest integer

let val = [1,2,56,80,100];
let result = largest_integer(&val);
println!("The Largest Integer is {:?}", result);


//calling the largest char
let vals = ['a','b','c','d'];
let result1 = largest_char(&vals);
println!("the largest char is {}", result1); 


//call for the generic


// let result3 = largest_integer(&val);
// println!("Generic Largest Integer is {}", result);

// let result4 = largest_char(&vals);
// println!("Generic largest char is {}", result4); 

//Generic.rs file call

let number: i32 = 33;
let text_str: &str = "Hello World";
print_debug(number);
print_debug(text_str);

print_display(number);
print_display(text_str);

//calling struct culture ,

// let culture_one = Culture{name:"Reggae".to_string()};
// println!("{:?}",culture_one);

let my_new_one: Point<f64> = Point::new(3.5,5.6);
println!("{}",my_new_one.get_y());


//calling swap function 
 let mut a = 1;
 let mut b = 2;
 swap(&mut a, &mut b);
 println!("The value of a is {} and b is {}",a,b);


 //calling the Vehicle.rs file 
// let x = Vehicle::new("KZD","KDA".to_string());
// let y = Vehicle::new("KAR", "KCF".to_string());

// x.swap(y);

// assert_eq!(x.customer_one,"KAR");

// println!("customer one {:?}",x);

//assign.rs call
let result = Game::new("Chess");
println!("{:?}",result);

// get_generic function

let val =  get_generic("James",21); 
println!("{}",val);



}  