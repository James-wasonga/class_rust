use crate::Coin::Penny;
use crate::Coin::Nickel;
use crate::Coin::Dime;
use crate::Coin::Quater;


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater,
}

fn main(){

    fn check_in_cent(coin: Coin)-> u8{
        match coin{
            //we can use curly bracket to make it easy

            Coin::Penny => {
                println!("The value of the penny is: ");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quater => 25,
        }
    }

    println!("The coin is {}",check_in_cent(Penny));
}