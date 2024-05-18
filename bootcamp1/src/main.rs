// // //function that returns a slice
// // fn main() {
    
// //     let my_string = String::from("Hello, Rust!");
// //     let slice = get_slice(&my_string, 7, 11);

// //     println!("Slice: {}", slice);
// // }

// // fn get_slice(s: &str, start: usize, end: usize) -> &str {
// //     &s[start..end]
// // }




// fn main(){
//     recussion_function(10);
// }

// fn recussion_function( mut par: i32){
// println!("number {par}");

// if par <= 10{
//     par +=1;
// }


mod fibonnaci;
use fibonnaci::check;

fn main(){
    let result = check(10);
    println!("{}",result);

}
