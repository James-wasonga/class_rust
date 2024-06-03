//create a program that consists of games
//each game should contain a game name, score and level,
//use the knowledge of struct to implement it 

pub struct Game{
    name: String,
    score: u32,
    level: u32,
}

impl Game{

    pub fn new(name: String, score: u32, level: u32) -> Self{
        return Game{
            name,score,level
        }
    } 

   pub fn return_level(&self) -> u32{
    return self.level;
}

   pub fn game_type(&self) -> &String{
       return &self.name;
    }

}