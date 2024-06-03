pub trait Summable{
    fn sum(&self) -> i32;
}
//trait in a type 
impl Summable for Vec<i32>{
    fn sum(&self) -> i32 {
        self.iter().sum()
    }
}

//generic inside a trait 

pub fn print_sum<T : Summable>(item: T){
    println!("The sum is {}",item.sum());
}