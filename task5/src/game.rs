//create a struct generic that has only one type and have implementation block that print anything
#[derive(Debug)]
pub struct Game <T>{
    name: T,
    //  score: (i32, i32)
}
impl <T> Game <T>{
  pub fn new(name: T) -> Self{
        Game{name}
    }

   pub  fn game_name(&self) -> &T{
       return &self.name;
    }

   
}

//create a function that takes two generic types and println

pub fn get_generic <T: std::fmt::Debug, Z: std::fmt::Debug> (name: T , age: Z) -> Z{

    // println!("{:?}, {:?}",name,age);

    return age;

}
