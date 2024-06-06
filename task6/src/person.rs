pub trait Describe{
    fn describe_person(&self) -> String;

    fn describe(&self) {
        println!("This describes traits");
    }


}



pub struct Person{
    pub name: String,
    pub age: i32,

}

impl Describe for Person{
  
     fn describe_person(&self) -> String{
        format!("The name is : {} and age is {}", self.name, self.age)
    }


    fn describe(&self){
        println!("God Help me Attain all my Goals, plans and dreams");
    }

   
}
