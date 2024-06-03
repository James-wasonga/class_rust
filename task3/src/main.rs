mod game_play;
use game_play::games::Game;

fn main(){

    // let game1 = Game{
    //     name: String::from("Chess"),
    //     score: 20,
    //     level: 4,
    // };

    // let game2 = Game{
    //     name: String::from("Ludo"),
    //     score: 10,
    //     level: 2,
    // };


    let game_result = Game::new(String::from("Chess"), 20, 4);
    // let game_result2 = game1.score;

    println!("The game type is {}", game_result.game_type());
    // println!("The game type is {}", game_result2);


}
