//create a trait called multiply
//have a implementation for vec[]
//implimentation should multiply the items in the vector

pub trait Multiply{
    fn multiply(&self) -> i32;
}

impl Multiply for Vec<i32>{
    fn multiply(&self) -> i32{
        self.iter().product()
    }

}

pub fn print_multiply<T: Multiply>(item: T){
    println!("The product is {}",item.multiply());
}