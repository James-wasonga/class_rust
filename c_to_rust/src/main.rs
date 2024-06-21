

// extern crate libc;

// extern{
//     fn double_input(input : libc::c_int) -> libc::c_int;
// }


// fn main(){
//     let my_int: i32 = 10;
//     let output = unsafe{
//         double_input(my_int);
//     }
//     println!("output {:#?}",output)
// }
use std::rc::Rc;
use core::cell::RefCell;
#[allow(unused)]
fn main(){
    let x: String = "Hello".to_string();
    let m = Rc::new(RefCell::new(x));//allows making a immutable variable to become mutable

    let a = Rc::clone(&m);
    let b = Rc::clone(&m);
    let c = Rc::clone(&m);

    // a.borrow_mut()
}