//using a knowledge of struct impl game char
//character : name , score, level 
mod game_chess;
use game_chess::game::Game;

fn main(){
    // let gme1 = game{
    //     name: String::from("Chess"),
    //     score: 12,
    //     level: 1,
    // };

    // let gme2 = game{
    //     name: String::from("Ludo"),
    //     score: 30,
    //     level: 2,

    // };
//print of the level that the play chess is
let mut result = Game::new(String::from("chess"),12,1);
    println!("The game chess is at level {}",result.return_level());
    println!("{}",result.return_level());
    result.increase_level();
    println!("{}",result.return_level());



}