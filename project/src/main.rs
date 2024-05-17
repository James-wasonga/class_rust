//managing book each as a name ,status , category 
//Using struct... implemnt

mod library;
use library::book;

fn main(){
// let Journal = book{
//     name: String::from("Perl"),
//     status: true || false,
//     category:String::from("life"),
// };
// let non_journal = book{
//     name:String::from("Blossom"),
//     status: true || false,
//     category:String::from("novel"),
// };

let Journal = book::new(String::from("Pearl"), true||false, String::from("life"));
let res = Journal.check_availability();

println!("The result is {}",res);
}

