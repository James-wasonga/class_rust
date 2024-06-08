// // fn main() {
// //     let x = Box::new(8);

// //     println!("x is {}",x);

// //     let x2 = *x * 10;
// //     println!("x2 is {}",x2);
// // }
// // use crate::List::Cons;
// // use crate::List::Nil;
// // std::fmt::Debug;

// enum List{
//     Cons(i32, Box<List>),
//     Nil,
// }
// use crate::List::{Cons, Nil};
// fn main(){
//     // let list = Cons(1,Box::new(Cons(2, Box::new(Cons(3,Box::new(Nil))))));
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    

//     print!("{}"list);
// }
// // struct Node{
// //     value: i32,
// //     next: Option<Box<Node>>

// // }
// // impl Node{
// //     fn new(value: i32) -> Self{
// //         Node{value,next: None}
// //     }

// //     fn append(&mut self, value: i32){
// //         match self.next{
// //             some(ref mut next_node => next_Node.append(value));
// //             None => self.next = Some(Box::new(Node::new(value))),
// //         }
// //     }
// //     fn print(&self){
// //         print("{}",self.value);
// //         if let Some(ref next_node) = self.next;
// //     }
// // }

// // fn main({
// //     let mut head = Node::new(1);
// //     head.append(2);
// //     head.append(3);
// //     head.append(5);

// //     head.print();

// // })


//Write a Rust function that takes a tuple (char, i32) and
// returns "Vowel" if the first element is a vowel ('a', 'e', 'i', 'o', 'u') and "Consonant" otherwise.
fn check_character(input: (char, i32)) ->&'static str{
  match input.0 {
     'a'|'e'|'i'|'o'|'u' => "vowel",
     _=> "Constants"
  }

}

fn main(){
    let char_result = vec![('g',1),('o',2),('u',3)];

    for i in char_result{
        println!("{}",check_character(i));
    }


    let input_1 = ('i',1);
    let input_2 = ('e', 2);
    let input_2 = ('a', 2);
    let input_2 = ('m', 2);




    println!("The alphabet is {}",check_alphabets(input_1));


    let result = find_less(10, 5);
    println!("{}",result);
}

//use logical or and a vector 

fn check_alphabets(input: (char, i32)) -> String{
    let my_vec = vec!['a','e','i','o','u'];

    if my_vec.iter().any(|t| *t == input.0){
        "vowel letter".to_string()
    }else{
        "constant".to_string()
    }
}

//create a generic function that takes two paramter and compares the two parameters returning the less/smaller of the two

fn find_less <T: std::cmp::PartialOrd> (par_1: T, par_2: T) -> T{
    if par_1 < par_2 {
        par_1
    }else {
        par_2
    }

    
}